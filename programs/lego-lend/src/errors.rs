use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("No default pubkey")]
    NoDefaultPubkey,
    #[msg("Max lltv exceeded")]
    MaxLltvExceeded,
    #[msg("Max fee exceeded")]
    MaxFeeExceeded,
    #[msg("Market not created")]
    MarketNotCreated,
    #[msg("Inconsistent input")]
    InconsistentInput,
}
