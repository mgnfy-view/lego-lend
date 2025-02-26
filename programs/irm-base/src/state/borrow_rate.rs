use anchor_lang::prelude::*;

use crate::errors::*;

#[account]
#[derive(InitSpace)]
pub struct BorrowRate {
    pub borrow_rate: u64,

    pub bump: u8,
}

impl BorrowRate {
    pub fn validate_borrow_rate(&self) -> Result<()> {
        require!(self.borrow_rate != 0, CustomErrors::ValueZero);

        Ok(())
    }
}
