use anchor_lang::prelude::*;

use crate::{constants::*, events::*, PlatformConfig};

#[derive(Accounts)]
pub struct SetFeeRecipient<'info> {
    #[account(
        address = platform_config.owner,
    )]
    pub owner: Signer<'info>,

    /// CHECK: The new fee recipient set by the owner.
    #[account()]
    pub new_fee_recipient: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [seeds::PLATFORM_CONFIG],
        bump,
    )]
    pub platform_config: Account<'info, PlatformConfig>,
}

impl SetFeeRecipient<'_> {
    pub fn set_fee_recipient(ctx: Context<SetFeeRecipient>) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;

        let new_fee_recipient = ctx.accounts.new_fee_recipient.key();

        platform_config.fee_recipient = new_fee_recipient;

        emit!(FeeRecipientSet {
            new_fee_recipient: new_fee_recipient
        });

        Ok(())
    }
}
