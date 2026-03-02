use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StrategyState {
    pub strategy_program: Pubkey,
    pub strategy_treasury: Pubkey,

    pub allocated_assets: u128,
    pub last_reported_value: u128,

    pub active: bool,
    pub bump: u8,
}
