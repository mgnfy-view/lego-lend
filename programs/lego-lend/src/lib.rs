use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "lego-lend",
    project_url: "https://github.com/mgnfy-view/lego-lend.git",
    contacts: "sahilgujrati12@gmail.com",
    policy: "",
    source_code: "https://github.com/mgnfy-view/lego-lend.git",
    preferred_languages: "en",
    auditors: ""
}

declare_id!("AQiKvjW1VD1ndzhKKTAzcVzqyoNCpotseY9Pt4iQUZgG");

#[program]
pub mod lego_lend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Initialize::initialize(ctx)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        TransferOwnership::transfer_ownership(ctx)
    }

    pub fn set_fee_recipient(ctx: Context<SetFeeRecipient>) -> Result<()> {
        SetFeeRecipient::set_fee_recipient(ctx)
    }
}
