use anchor_lang::prelude::*;

#[event]
pub struct Initialized {
    pub owner: Pubkey,
    pub fee_recipient: Pubkey,
}
