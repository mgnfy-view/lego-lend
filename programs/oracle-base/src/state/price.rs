use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Price {
    pub price: u64,

    pub bump: u8,
}
