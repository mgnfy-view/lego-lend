use anchor_lang::prelude::*;

#[event]
pub struct Initialized {
    pub borrow_rate: u64,
}

#[event]
pub struct BorrowRateUpdated {
    pub borrow_rate: u64,
}
