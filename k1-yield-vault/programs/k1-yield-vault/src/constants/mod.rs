use anchor_lang::prelude::*;

#[constant]
pub const ASSET_DECIMALS: u8 = 6;
#[constant]
pub const SHARE_DECIMALS: u8 = 9;

#[constant]
pub const ASSET_PRECISION: u128 = 1_000_000;
#[constant]
pub const SHARE_PRECISION: u128 = 1_000_000_000;
#[constant]
pub const PRICE_PRECISION: u128 = 1_000_000_000_000_000_000;

#[constant]
pub const BPS_DENOMINATOR: u128 = 10_000;

#[constant]
pub const VAULT_SEED: &str = "vault";
#[constant]
pub const VAULT_TREASURY_SEED: &str = "vault_treasury";
#[constant]
pub const SHARE_MINT_AUTH_SEED: &str = "share_mint_auth";
#[constant]
pub const STRATEGY_SEED: &str = "strategy";
