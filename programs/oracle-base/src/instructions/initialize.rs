use anchor_lang::prelude::*;

use crate::{constants::*, events::*, Price};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = general::ANCHOR_DISCRIMINATOR_SIZE + Price::INIT_SPACE,
        seeds = [seeds::PRICE],
        bump,
    )]
    pub price: Account<'info, Price>,

    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn initialize(ctx: Context<Initialize>, price: u64) -> Result<()> {
        let price_account = &mut ctx.accounts.price;

        price_account.price = price;

        price_account.bump = ctx.bumps.price;

        price_account.validate_price()?;

        emit!(Initialized { price: price });

        Ok(())
    }
}
