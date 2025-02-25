use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Value zero")]
    ValueZero,
}
