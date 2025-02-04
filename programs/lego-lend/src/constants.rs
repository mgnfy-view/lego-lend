use anchor_lang::prelude::*;

pub mod seeds {
    use super::*;

    #[constant]
    pub const PLATFORM_CONFIG: &[u8] = b"platform_config";
}

pub mod general {
    use super::*;

    #[constant]
    pub const BPS: u16 = 10_000;
    #[constant]
    pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
}
