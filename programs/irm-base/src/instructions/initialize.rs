use anchor_lang::prelude::*;

use crate::{constants::*, errors::*, events::*, BorrowRate};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = general::ANCHOR_DISCRIMINATOR_SIZE + BorrowRate::INIT_SPACE,
        seeds = [seeds::BORROW_RATE],
        bump,
    )]
    pub borrow_rate: Account<'info, BorrowRate>,

    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn initialize(ctx: Context<Initialize>, borrow_rate: u64) -> Result<()> {
        let borrow_rate_account = &mut ctx.accounts.borrow_rate;

        require!(borrow_rate != 0, CustomErrors::ValueZero);

        borrow_rate_account.borrow_rate = borrow_rate;

        borrow_rate_account.bump = ctx.bumps.borrow_rate;

        emit!(Initialized {
            borrow_rate: borrow_rate
        });

        Ok(())
    }
}
