use anchor_lang::prelude::*;

use crate::{events::*, BorrowRate};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account()]
    pub borrow_rate: Account<'info, BorrowRate>,

    /// CHECK: Additional account that may or may not be used by the by the irm.
    #[account()]
    pub supplemental_irm_account_1: UncheckedAccount<'info>,

    /// CHECK: Additional account that may or may not be used by the by the irm.
    #[account()]
    pub supplemental_irm_account_2: UncheckedAccount<'info>,
}

impl Update<'_> {
    pub fn update(
        ctx: Context<Update>,
        _total_supply_assets: u64,
        _total_supply_shares: u64,
        _total_borrow_assets: u64,
        _total_borrow_shares: u64,
        _last_update: u64,
        _fee: u64,
    ) -> Result<()> {
        emit!(BorrowRateUpdated {
            borrow_rate: ctx.accounts.borrow_rate.borrow_rate
        });

        Ok(())
    }
}
