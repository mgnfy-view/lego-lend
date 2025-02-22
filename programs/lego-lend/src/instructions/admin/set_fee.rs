use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{constants::*, errors::*, FeeSet, Market, PlatformConfig};

// Note: Accrue interest before changing fees

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(
        address = platform_config.owner
    )]
    pub owner: Signer<'info>,

    #[account(
        seeds = [seeds::PLATFORM_CONFIG],
        bump,
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account()]
    pub loan_token: InterfaceAccount<'info, Mint>,

    #[account()]
    pub collateral_token: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [seeds::MARKET, loan_token.key().as_ref(), collateral_token.key().as_ref()],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,

    pub system_program: Program<'info, System>,
}

impl SetFee<'_> {
    pub fn set_fee(ctx: Context<SetFee>, new_fee: u64) -> Result<()> {
        require!(new_fee <= general::MAX_FEE, CustomErrors::MaxFeeExceeded);

        ctx.accounts.market.fee = new_fee;

        emit!(FeeSet { new_fee: new_fee });

        Ok(())
    }
}
