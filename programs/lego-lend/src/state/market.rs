use anchor_lang::prelude::*;

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
}

#[derive(Clone, InitSpace, AnchorSerialize, AnchorDeserialize)]
pub struct MarketParams {
    pub loan_token: Pubkey,
    pub collateral_token: Pubkey,
    pub oracle: Pubkey,
    pub irm: Pubkey,
    pub lltv: u64,
}
