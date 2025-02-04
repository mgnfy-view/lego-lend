use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("No default pubkey")]
    NoDefaultPubkey,
}
