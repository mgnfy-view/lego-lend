use anchor_lang::prelude::*;

#[event]
pub struct Initialized {
    pub price: u64,
}

#[event]
pub struct PriceUpdated {
    pub price: u64,
}
