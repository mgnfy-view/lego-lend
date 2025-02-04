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
