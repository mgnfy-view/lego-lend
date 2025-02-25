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
    name: "irm-base",
    project_url: "https://github.com/mgnfy-view/lego-lend.git",
    contacts: "sahilgujrati12@gmail.com",
    policy: "",
    source_code: "https://github.com/mgnfy-view/lego-lend.git",
    preferred_languages: "en",
    auditors: ""
}

declare_id!("HqfsZmdbtUiG5cv2HDhB4c7Ewjoxqe2NnEWzA9HJyCEQ");

#[program]
pub mod irm_base {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, borrow_rate: u64) -> Result<()> {
        Initialize::initialize(ctx, borrow_rate)
    }

    pub fn update(
        ctx: Context<Update>,
        total_supply_assets: u64,
        total_supply_shares: u64,
        total_borrow_assets: u64,
        total_borrow_shares: u64,
        last_update: u64,
        fee: u64,
    ) -> Result<()> {
        Update::update(
            ctx,
            total_supply_assets,
            total_supply_shares,
            total_borrow_assets,
            total_borrow_shares,
            last_update,
            fee,
        )
    }
}
