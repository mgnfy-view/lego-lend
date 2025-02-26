use anchor_lang::prelude::*;

use crate::{constants, errors::*};

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub market_params: MarketParams,

    pub total_supply_assets: u64,
    pub total_supply_shares: u64,
    pub total_borrow_assets: u64,
    pub total_borrow_shares: u64,
    pub last_update: u64,
    pub fee: u64,

    pub bump: u8,
    pub loan_token_account_bump: u8,
    pub collateral_token_account_bump: u8,
}

impl Market {
    pub fn validate_lltv(&self) -> Result<()> {
        require!(
            self.market_params.lltv <= constants::general::E9,
            CustomErrors::MaxLltvExceeded
        );

        Ok(())
    }

    pub fn validate_fee(&self) -> Result<()> {
        require!(
            self.fee <= constants::general::MAX_FEE,
            CustomErrors::MaxFeeExceeded
        );

        Ok(())
    }

    pub fn validate_last_update_timestamp(&self) -> Result<()> {
        require!(self.last_update != 0, CustomErrors::MarketNotCreated);

        Ok(())
    }
}

#[derive(Clone, InitSpace, AnchorSerialize, AnchorDeserialize)]
pub struct MarketParams {
    pub loan_token: Pubkey,
    pub collateral_token: Pubkey,
    pub oracle: Pubkey,
    pub irm: Pubkey,
    pub lltv: u64,
}
