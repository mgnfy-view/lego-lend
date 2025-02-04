use anchor_lang::prelude::*;

use crate::{constants::*, errors::*, Initialized, PlatformConfig};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: The fee recipient set by the admin.
    #[account()]
    pub fee_recipient: AccountInfo<'info>,

    #[account(
        init,
        payer = owner,
        space = general::ANCHOR_DISCRIMINATOR_SIZE + PlatformConfig::INIT_SPACE,
        seeds = [seeds::PLATFORM_CONFIG],
        bump,
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;

        let owner = ctx.accounts.owner.key();
        let fee_recipient = ctx.accounts.fee_recipient.key();

        require!(
            owner != Pubkey::default() && fee_recipient != Pubkey::default(),
            CustomErrors::NoDefaultPubkey,
        );

        platform_config.owner = owner;
        platform_config.fee_recipient = fee_recipient;

        emit!(Initialized {
            owner: owner,
            fee_recipient: fee_recipient
        });

        Ok(())
    }
}
