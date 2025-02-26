use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{constants, events::*, utils, Market, PlatformConfig, Position};

#[derive(Accounts)]
pub struct AccrueInterest<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,

    #[account(
        seeds = [constants::seeds::PLATFORM_CONFIG],
        bump = platform_config.bump,
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account()]
    pub loan_token: InterfaceAccount<'info, Mint>,

    #[account()]
    pub collateral_token: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [constants::seeds::MARKET, loan_token.key().as_ref(), collateral_token.key().as_ref()],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,

    /// CHECK: Account that serves as the oracle for the market.
    #[account(
        address = market.market_params.oracle,
        executable
    )]
    pub oracle: AccountInfo<'info>,

    #[account()]
    pub price: Account<'info, oracle_base::Price>,

    /// CHECK: Additional account that may or may not be used by the by the oracle.
    #[account()]
    pub supplemental_oracle_account_1: UncheckedAccount<'info>,

    /// CHECK: Additional account that may or may not be used by the by the oracle.
    #[account()]
    pub supplemental_oracle_account_2: UncheckedAccount<'info>,

    /// CHECK: Account that serves as the interest rate model for the market.
    #[account(
        address = market.market_params.irm,
        executable
    )]
    pub irm: AccountInfo<'info>,

    #[account()]
    pub borrow_rate: Account<'info, irm_base::BorrowRate>,

    /// CHECK: Additional account that may or may not be used by the by the irm.
    #[account()]
    pub supplemental_irm_account_1: UncheckedAccount<'info>,

    /// CHECK: Additional account that may or may not be used by the by the irm.
    #[account()]
    pub supplemental_irm_account_2: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = caller,
        space = constants::general::ANCHOR_DISCRIMINATOR_SIZE + Position::INIT_SPACE,
        seeds = [constants::seeds::POSITION, market.key().as_ref(), platform_config.fee_recipient.as_ref()],
        bump,
    )]
    pub fee_recipient_position: Account<'info, Position>,

    pub system_program: Program<'info, System>,
}

impl AccrueInterest<'_> {
    pub fn accrue_interest(ctx: Context<AccrueInterest>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let price = &mut ctx.accounts.price;
        let borrow_rate_account = &mut ctx.accounts.borrow_rate;

        market.validate_last_update_timestamp()?;

        let current_timestamp = Clock::get()?.unix_timestamp as u64;
        let elapsed_time = current_timestamp - market.last_update;
        if elapsed_time == 0 {
            return Ok(());
        }

        oracle_base::cpi::update(CpiContext::new(
            ctx.accounts.oracle.to_account_info(),
            oracle_base::cpi::accounts::Update {
                price: price.to_account_info(),
                supplemental_oracle_account_1: ctx
                    .accounts
                    .supplemental_oracle_account_1
                    .to_account_info(),
                supplemental_oracle_account_2: ctx
                    .accounts
                    .supplemental_oracle_account_1
                    .to_account_info(),
            },
        ))?;
        price.reload()?;

        irm_base::cpi::update(
            CpiContext::new(
                ctx.accounts.irm.to_account_info(),
                irm_base::cpi::accounts::Update {
                    borrow_rate: borrow_rate_account.to_account_info(),
                    supplemental_irm_account_1: ctx
                        .accounts
                        .supplemental_irm_account_1
                        .to_account_info(),
                    supplemental_irm_account_2: ctx
                        .accounts
                        .supplemental_irm_account_2
                        .to_account_info(),
                },
            ),
            market.total_supply_assets,
            market.total_supply_shares,
            market.total_borrow_assets,
            market.total_borrow_shares,
            market.last_update,
            market.fee,
        )?;
        borrow_rate_account.reload()?;

        let borrow_rate = borrow_rate_account.borrow_rate;
        let interest = u64::try_from(utils::math::l_mul_down(
            &(market.total_borrow_assets as u128),
            &utils::math::l_taylor_compounded(&(borrow_rate as u128), &(elapsed_time as u128)),
        ))
        .unwrap();
        market.total_borrow_assets += interest;
        market.total_supply_assets += interest;

        let fee_shares = 0;
        if market.fee != 0 {
            let fee_amount = utils::math::l_mul_down(&(interest as u128), &(market.fee as u128));
            let fee_shares = u64::try_from(utils::math::to_shares_down(
                &fee_amount,
                &(market.total_supply_assets as u128 - fee_amount),
                &(market.total_supply_shares as u128),
            ))
            .unwrap();

            ctx.accounts.fee_recipient_position.supply_shares += fee_shares;
            market.total_supply_shares += fee_shares;
        }

        market.last_update = current_timestamp;

        emit!(InterestAccrued {
            market: market.key(),
            borrow_rate: borrow_rate,
            interest: interest,
            fee_shares: fee_shares
        });

        Ok(())
    }
}
