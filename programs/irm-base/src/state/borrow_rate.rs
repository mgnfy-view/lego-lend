use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BorrowRate {
    pub borrow_rate: u64,

    pub bump: u8,
}
