use anchor_lang::prelude::*;

pub mod seeds {
    use super::*;

    #[constant]
    pub const PLATFORM_CONFIG: &[u8] = b"platform_config";

    #[constant]
    pub const MARKET: &[u8] = b"market";

    #[constant]
    pub const VAULT: &[u8] = b"vault";

    #[constant]
    pub const POSITION: &[u8] = b"position";
}

pub mod general {
    use super::*;

    #[constant]
    pub const BPS: u16 = 10_000;

    #[constant]
    pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

    #[constant]
    pub const E9: u64 = 1_000_000_000;

    #[constant]
    pub const MAX_FEE: u64 = 250_000_000;

    #[constant]
    pub const VIRTUAL_ASSETS: u64 = 1;

    #[constant]
    pub const VIRTUAL_SHARES: u64 = 1_000;
}
