use crate::errors::*;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    pub owner: Pubkey,
    pub fee_recipient: Pubkey,

    pub bump: u8,
}

impl PlatformConfig {
    pub fn validate_fee_recipient(&self) -> Result<()> {
        require!(
            self.fee_recipient != Pubkey::default(),
            CustomErrors::NoDefaultPubkey
        );

        Ok(())
    }
}
