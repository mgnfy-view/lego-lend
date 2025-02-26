use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{constants, utils, Market, MarketCreated, PlatformConfig};

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(
        mut,
        address = platform_config.owner,
    )]
    pub creator: Signer<'info>,

    #[account(
        seeds = [constants::seeds::PLATFORM_CONFIG],
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
        space = constants::general::ANCHOR_DISCRIMINATOR_SIZE + Market::INIT_SPACE,
        seeds = [constants::seeds::MARKET, loan_token.key().as_ref(), collateral_token.key().as_ref()],
        bump,
    )]
    pub market: Account<'info, Market>,

    /// CHECK: The loan token account to be created for the market. Creating this within the
    /// instruction to avoid stack overflow.
    #[account(
        mut,
        seeds = [constants::seeds::VAULT, market.key().as_ref(), loan_token.key().as_ref()],
        bump,
    )]
    pub loan_token_account: UncheckedAccount<'info>,

    /// CHECK: The collateral token account to be created for the market. Creating this within the
    /// instruction to avoid stack overflow.
    #[account(
        mut,
        seeds = [constants::seeds::VAULT, market.key().as_ref(), collateral_token.key().as_ref()],
        bump,
    )]
    pub collateral_token_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl CreateMarket<'_> {
    pub fn create_market(ctx: Context<CreateMarket>, lltv: u64, fee: u64) -> Result<()> {
        let market = &mut ctx.accounts.market;

        let loan_token_pubkey = ctx.accounts.loan_token.key();
        let collateral_token_pubkey = ctx.accounts.collateral_token.key();
        let oracle_pubkey = ctx.accounts.oracle.key();
        let irm_pubkey = ctx.accounts.irm.key();
        let market_pubkey = market.key();

        let loan_token_account_bump = &[ctx.bumps.loan_token_account];
        let loan_token_account_signer = &[
            constants::seeds::VAULT,
            market_pubkey.as_ref(),
            loan_token_pubkey.as_ref(),
            loan_token_account_bump,
        ][..];
        utils::general::create_token_account(
            &market.to_account_info(),
            &ctx.accounts.creator.to_account_info(),
            &ctx.accounts.loan_token_account.to_account_info(),
            &ctx.accounts.loan_token.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.accounts.token_program.to_account_info(),
            loan_token_account_signer,
        )?;

        let collateral_token_account_bump = &[ctx.bumps.collateral_token_account];
        let collateral_token_account_signer = &[
            constants::seeds::VAULT,
            market_pubkey.as_ref(),
            collateral_token_pubkey.as_ref(),
            collateral_token_account_bump,
        ][..];
        utils::general::create_token_account(
            &market.to_account_info(),
            &ctx.accounts.creator.to_account_info(),
            &ctx.accounts.collateral_token_account.to_account_info(),
            &ctx.accounts.collateral_token.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
            &ctx.accounts.token_program.to_account_info(),
            collateral_token_account_signer,
        )?;

        market.market_params.loan_token = loan_token_pubkey;
        market.market_params.collateral_token = collateral_token_pubkey;
        market.market_params.oracle = oracle_pubkey;
        market.market_params.irm = irm_pubkey;
        market.market_params.lltv = lltv;

        market.fee = fee;
        market.last_update = u64::try_from(Clock::get()?.unix_timestamp).unwrap();

        market.bump = ctx.bumps.market;
        market.loan_token_account_bump = ctx.bumps.loan_token_account;
        market.collateral_token_account_bump = ctx.bumps.collateral_token_account;

        market.validate_lltv()?;
        market.validate_fee()?;

        emit!(MarketCreated {
            market: market.key(),
            loan_token: loan_token_pubkey,
            collateral_token: collateral_token_pubkey,
            oracle: oracle_pubkey,
            irm: irm_pubkey,
            lltv: lltv,
            fee: fee
        });

        Ok(())
    }
}
