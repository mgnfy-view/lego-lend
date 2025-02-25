use anchor_lang::prelude::*;

use crate::{constants::*, events::*, Price};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [seeds::PRICE],
        bump = price.bump,
    )]
    pub price: Account<'info, Price>,

    /// CHECK: Additional account that may or may not be used by the by the oracle.
    #[account()]
    pub supplemental_oracle_account_1: UncheckedAccount<'info>,

    /// CHECK: Additional account that may or may not be used by the by the oracle.
    #[account()]
    pub supplemental_oracle_account_2: UncheckedAccount<'info>,
}

impl Update<'_> {
    pub fn update(ctx: Context<Update>) -> Result<()> {
        emit!(PriceUpdated {
            price: ctx.accounts.price.price
        });

        Ok(())
    }
}
