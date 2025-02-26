use anchor_lang::prelude::*;

use crate::errors::*;

#[account]
#[derive(InitSpace)]
pub struct Price {
    pub price: u64,

    pub bump: u8,
}

impl Price {
    pub fn validate_price(&self) -> Result<()> {
        require!(self.price != 0, CustomErrors::ValueZero);

        Ok(())
    }
}
