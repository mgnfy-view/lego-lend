use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::InitializeAccount3,
    token_interface::{initialize_account3, Mint, TokenInterface},
};

use crate::{constants::*, errors::*, Market, MarketCreated, PlatformConfig};

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(
        mut,
        address = platform_config.owner
    )]
    pub creator: Signer<'info>,

    #[account(
        seeds = [seeds::PLATFORM_CONFIG],
        bump = platform_config.bump,
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account()]
    pub loan_token: InterfaceAccount<'info, Mint>,

    #[account()]
    pub collateral_token: InterfaceAccount<'info, Mint>,

    /// CHECK: Account that serves as the oracle for the market.
    #[account()]
    pub oracle: AccountInfo<'info>,

    /// CHECK: Account that serves as the interest rate model for the market.
    #[account()]
    pub irm: AccountInfo<'info>,

    #[account(
        init,
        payer = creator,
        space = general::ANCHOR_DISCRIMINATOR_SIZE + Market::INIT_SPACE,
        seeds = [seeds::MARKET, loan_token.key().as_ref(), collateral_token.key().as_ref()],
        bump,
    )]
    pub market: Account<'info, Market>,

    /// CHECK: The loan token account to be created for the market. Creating this within the
    /// instruction to avoid stack overflow.
    #[account(
        mut,
        seeds = [seeds::VAULT, market.key().as_ref(), loan_token.key().as_ref()],
        bump,
    )]
    pub loan_token_account: UncheckedAccount<'info>,

    /// CHECK: The collateral token account to be created for the market. Creating this within the
    /// instruction to avoid stack overflow.
    #[account(
        mut,
        seeds = [seeds::VAULT, market.key().as_ref(), collateral_token.key().as_ref()],
        bump,
    )]
    pub collateral_token_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl CreateMarket<'_> {
    pub fn create_market(ctx: Context<CreateMarket>, lltv: u64, fee: u64) -> Result<()> {
        require!(lltv <= general::E9, CustomErrors::MaxLltvExceeded);
        require!(fee <= general::MAX_FEE, CustomErrors::MaxFeeExceeded);

        let market = &mut ctx.accounts.market;

        let loan_token = ctx.accounts.loan_token.key();
        let collateral_token = ctx.accounts.collateral_token.key();
        let oracle = ctx.accounts.oracle.key();
        let irm = ctx.accounts.irm.key();

        initialize_account3(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeAccount3 {
                account: ctx.accounts.loan_token_account.to_account_info(),
                mint: ctx.accounts.loan_token.to_account_info(),
                authority: market.to_account_info(),
            },
        ))?;
        initialize_account3(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeAccount3 {
                account: ctx.accounts.collateral_token_account.to_account_info(),
                mint: ctx.accounts.collateral_token.to_account_info(),
                authority: market.to_account_info(),
            },
        ))?;

        market.market_params.loan_token = loan_token;
        market.market_params.collateral_token = collateral_token;
        market.market_params.oracle = oracle;
        market.market_params.irm = irm;
        market.market_params.lltv = lltv;

        market.fee = fee;
        market.last_update = Clock::get()?.unix_timestamp as u64;

        market.bump = ctx.bumps.market;

        emit!(MarketCreated {
            market: market.key(),
            loan_token: loan_token,
            collateral_token: collateral_token,
            oracle: oracle,
            irm: irm,
            lltv: lltv,
            fee: fee
        });

        Ok(())
    }
}
