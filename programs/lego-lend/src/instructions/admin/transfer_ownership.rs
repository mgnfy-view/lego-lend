use anchor_lang::prelude::*;

use crate::{constants::*, events::*, PlatformConfig};

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        address = platform_config.owner,
    )]
    pub owner: Signer<'info>,

    #[account()]
    pub new_owner: Signer<'info>,

    #[account(
        mut,
        seeds = [seeds::PLATFORM_CONFIG],
        bump,
    )]
    pub platform_config: Account<'info, PlatformConfig>,
}

impl TransferOwnership<'_> {
    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;

        let new_owner = ctx.accounts.new_owner.key();

        platform_config.owner = new_owner;

        emit!(OwnershipTransferred {
            new_owner: new_owner
        });

        Ok(())
    }
}
