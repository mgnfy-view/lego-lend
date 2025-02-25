use anchor_lang::prelude::*;

pub mod seeds {
    use super::*;

    #[constant]
    pub const PRICE: &[u8] = b"price";
}

pub mod general {
    use super::*;

    #[constant]
    pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
}
