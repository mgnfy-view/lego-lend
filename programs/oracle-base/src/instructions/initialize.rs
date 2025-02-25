use anchor_lang::prelude::*;

use crate::{constants::*, errors::*, events::*, Price};

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

        require!(price != 0, CustomErrors::ValueZero);

        price_account.price = price;

        price_account.bump = ctx.bumps.price;

        emit!(Initialized { price: price });

        Ok(())
    }
}
