use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    pub owner: Pubkey,
    pub fee_recipient: Pubkey,
}
