use anchor_lang::prelude::*;
#[cfg(not(feature = "no-entrypoint"))]
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
    name: "oracle-base",
    project_url: "https://github.com/mgnfy-view/lego-lend.git",
    contacts: "sahilgujrati12@gmail.com",
    policy: "",
    source_code: "https://github.com/mgnfy-view/lego-lend.git",
    preferred_languages: "en",
    auditors: ""
}

declare_id!("L6eUquiyrvQfbdKVn76qMiH8ZgbJFBFAjY1fjoapDEg");

#[program]
pub mod oracle_base {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, price: u64) -> Result<()> {
        Initialize::initialize(ctx, price)
    }

    pub fn update(ctx: Context<Update>) -> Result<()> {
        Update::update(ctx)
    }
}
