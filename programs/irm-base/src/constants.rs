use anchor_lang::prelude::*;

pub mod seeds {
    use super::*;

    #[constant]
    pub const BORROW_RATE: &[u8] = b"borrow_rate";
}

pub mod general {
    use super::*;

    #[constant]
    pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
}
