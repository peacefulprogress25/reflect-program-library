use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::K1Error;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub usdc_mint: Pubkey,
    pub share_mint: Pubkey,
    pub treasury: Pubkey,
    pub governance: Pubkey,
    pub reporter: Pubkey,

    pub total_shares: u128,
    pub offchain_nav: u128,
    pub strategy_value_sum: u128,

    pub high_water_mark: u128,
    pub last_total_assets: u128,

    pub mint_fee_bps: u16,
    pub withdrawal_fee_bps: u16,
    pub performance_fee_bps: u16,

    pub paused: bool,

    pub strategy_count: u8,
    pub bump: u8,
}

impl Vault {
    pub fn check_not_paused(&self) -> Result<()> {
        require!(!self.paused, K1Error::VaultPaused);
        Ok(())
    }

    pub fn total_assets(&self, idle_assets: u64) -> Result<u128> {
        let idle = idle_assets as u128;
        idle.checked_add(self.strategy_value_sum)
            .ok_or(K1Error::MathOverflow.into())?
            .checked_add(self.offchain_nav)
            .ok_or(K1Error::MathOverflow.into())
    }

    pub fn share_price(&self, total_assets: u128) -> Result<u128> {
        if self.total_shares == 0 {
            return Ok(PRICE_PRECISION);
        }
        total_assets
            .checked_mul(PRICE_PRECISION)
            .ok_or(K1Error::MathOverflow.into())?
            .checked_div(self.total_shares)
            .ok_or(K1Error::DivisionByZero.into())
    }

    pub fn compute_deposit_shares(&self, amount: u64, total_assets_before: u128) -> Result<u128> {
        let amount_u128 = amount as u128;
        if self.total_shares == 0 {
            return amount_u128
                .checked_mul(SHARE_PRECISION)
                .ok_or(K1Error::MathOverflow.into())?
                .checked_div(ASSET_PRECISION)
                .ok_or(K1Error::DivisionByZero.into());
        }

        amount_u128
            .checked_mul(self.total_shares)
            .ok_or(K1Error::MathOverflow.into())?
            .checked_div(total_assets_before)
            .ok_or(K1Error::DivisionByZero.into())
    }

    pub fn apply_mint_fee(&self, shares_to_mint: u128) -> Result<(u128, u128)> {
        let fee_shares = shares_to_mint
            .checked_mul(self.mint_fee_bps as u128)
            .ok_or(K1Error::MathOverflow)?
            .checked_div(BPS_DENOMINATOR)
            .ok_or(K1Error::DivisionByZero)?;
        let user_shares = shares_to_mint
            .checked_sub(fee_shares)
            .ok_or(K1Error::MathOverflow)?;
        Ok((user_shares, fee_shares))
    }

    pub fn compute_withdraw_assets(&self, shares_to_burn: u64, total_assets: u128) -> Result<u128> {
        let shares = shares_to_burn as u128;
        shares
            .checked_mul(total_assets)
            .ok_or(K1Error::MathOverflow.into())?
            .checked_div(self.total_shares)
            .ok_or(K1Error::DivisionByZero.into())
    }

    pub fn apply_withdraw_fee(&self, assets_out: u128) -> Result<(u128, u128)> {
        let fee_assets = assets_out
            .checked_mul(self.withdrawal_fee_bps as u128)
            .ok_or(K1Error::MathOverflow)?
            .checked_div(BPS_DENOMINATOR)
            .ok_or(K1Error::DivisionByZero)?;
        let user_assets = assets_out
            .checked_sub(fee_assets)
            .ok_or(K1Error::MathOverflow)?;
        Ok((user_assets, fee_assets))
    }

    pub fn fee_assets_to_shares(
        &self,
        fee_assets: u128,
        total_assets: u128,
        assets_out: u128,
    ) -> Result<u128> {
        let denominator = total_assets
            .checked_sub(assets_out)
            .ok_or(K1Error::MathOverflow)?;
        fee_assets
            .checked_mul(self.total_shares)
            .ok_or(K1Error::MathOverflow.into())?
            .checked_div(denominator)
            .ok_or(K1Error::DivisionByZero.into())
    }

    pub fn apply_performance_fee(&mut self, total_assets: u128) -> Result<u128> {
        if total_assets <= self.high_water_mark {
            self.last_total_assets = total_assets;
            return Ok(0);
        }

        let profit = total_assets
            .checked_sub(self.high_water_mark)
            .ok_or(K1Error::MathOverflow)?;

        let fee_assets = profit
            .checked_mul(self.performance_fee_bps as u128)
            .ok_or(K1Error::MathOverflow)?
            .checked_div(BPS_DENOMINATOR)
            .ok_or(K1Error::DivisionByZero)?;

        if fee_assets == 0 || self.total_shares == 0 {
            self.high_water_mark = total_assets;
            self.last_total_assets = total_assets;
            return Ok(0);
        }

        let denominator = total_assets
            .checked_sub(fee_assets)
            .ok_or(K1Error::MathOverflow)?;

        let fee_shares = fee_assets
            .checked_mul(self.total_shares)
            .ok_or(K1Error::MathOverflow)?
            .checked_div(denominator)
            .ok_or(K1Error::DivisionByZero)?;

        self.total_shares = self
            .total_shares
            .checked_add(fee_shares)
            .ok_or(K1Error::MathOverflow)?;
        self.high_water_mark = total_assets;
        self.last_total_assets = total_assets;

        Ok(fee_shares)
    }

    pub fn validate_fees(&self) -> Result<()> {
        require!(
            (self.mint_fee_bps as u128) <= BPS_DENOMINATOR,
            K1Error::InvalidFeeBps
        );
        require!(
            (self.withdrawal_fee_bps as u128) <= BPS_DENOMINATOR,
            K1Error::InvalidFeeBps
        );
        require!(
            (self.performance_fee_bps as u128) <= BPS_DENOMINATOR,
            K1Error::InvalidFeeBps
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vault() -> Vault {
        Vault {
            usdc_mint: Pubkey::new_unique(),
            share_mint: Pubkey::new_unique(),
            treasury: Pubkey::new_unique(),
            governance: Pubkey::new_unique(),
            reporter: Pubkey::new_unique(),
            total_shares: 0,
            offchain_nav: 0,
            strategy_value_sum: 0,
            high_water_mark: 0,
            last_total_assets: 0,
            mint_fee_bps: 100,
            withdrawal_fee_bps: 50,
            performance_fee_bps: 2000,
            paused: false,
            strategy_count: 0,
            bump: 255,
        }
    }

    #[test]
    fn deposit_first_mints_precision_adjusted_shares() {
        let v = vault();
        let shares = v.compute_deposit_shares(1_000_000, 0).unwrap();
        assert_eq!(shares, 1_000_000_000);
    }

    #[test]
    fn withdraw_fee_is_applied() {
        let mut v = vault();
        v.total_shares = 1_000_000_000;
        let assets_out = v.compute_withdraw_assets(100_000_000, 1_000_000).unwrap();
        let (user_assets, fee_assets) = v.apply_withdraw_fee(assets_out).unwrap();
        assert_eq!(assets_out, 100_000);
        assert_eq!(fee_assets, 500);
        assert_eq!(user_assets, 99_500);
    }

    #[test]
    fn performance_fee_on_profit_mints_shares() {
        let mut v = vault();
        v.total_shares = 1_000_000_000;
        v.high_water_mark = 1_000_000;
        let fee_shares = v.apply_performance_fee(1_200_000).unwrap();
        assert!(fee_shares > 0);
        assert_eq!(v.high_water_mark, 1_200_000);
    }

    #[test]
    fn performance_fee_on_loss_does_nothing() {
        let mut v = vault();
        v.total_shares = 1_000_000_000;
        v.high_water_mark = 1_000_000;
        let fee_shares = v.apply_performance_fee(900_000).unwrap();
        assert_eq!(fee_shares, 0);
        assert_eq!(v.high_water_mark, 1_000_000);
    }

    #[test]
    fn offchain_nav_included_in_total_assets() {
        let mut v = vault();
        v.offchain_nav = 300_000;
        v.strategy_value_sum = 200_000;
        let total = v.total_assets(500_000).unwrap();
        assert_eq!(total, 1_000_000);
    }

    #[test]
    fn pause_blocks_operations() {
        let mut v = vault();
        v.paused = true;
        assert!(v.check_not_paused().is_err());
    }

    #[test]
    fn strategy_profit_updates_share_price_inputs() {
        let mut v = vault();
        v.total_shares = 1_000_000_000;
        let before = v.total_assets(1_000_000).unwrap();
        v.strategy_value_sum = 200_000;
        let after = v.total_assets(1_000_000).unwrap();
        assert!(after > before);
    }
}
