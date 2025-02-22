use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub supply_shares: u64,
    pub borrow_shares: u64,
    pub collateral: u64,

    pub bump: u8,
}
