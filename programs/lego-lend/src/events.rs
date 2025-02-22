use anchor_lang::prelude::*;

#[event]
pub struct Initialized {
    pub owner: Pubkey,
    pub fee_recipient: Pubkey,
}

#[event]
pub struct OwnershipTransferred {
    pub new_owner: Pubkey,
}

#[event]
pub struct FeeRecipientSet {
    pub new_fee_recipient: Pubkey,
}

#[event]
pub struct MarketCreated {
    pub market: Pubkey,
    pub loan_token: Pubkey,
    pub collateral_token: Pubkey,
    pub oracle: Pubkey,
    pub irm: Pubkey,
    pub lltv: u64,
    pub fee: u64,
}

#[event]
pub struct FeeSet {
    pub new_fee: u64,
}
