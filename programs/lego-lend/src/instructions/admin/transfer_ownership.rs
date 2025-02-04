use anchor_lang::prelude::*;

use crate::{constants::*, errors::*, events::*, PlatformConfig};

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

    pub system_program: Program<'info, System>,
}

impl TransferOwnership<'_> {
    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;

        let new_owner = ctx.accounts.new_owner.key();

        require!(
            new_owner != Pubkey::default(),
            CustomErrors::NoDefaultPubkey,
        );

        platform_config.owner = new_owner;

        emit!(OwnershipTransferred {
            new_owner: new_owner
        });

        Ok(())
    }
}
