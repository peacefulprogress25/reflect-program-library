use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::{StrategyState, Vault};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DeactivateStrategyArgs {
    pub vault_id: u64,
    pub strategy_id: u64,
}

pub fn deactivate_strategy(
    ctx: Context<DeactivateStrategy>,
    _args: DeactivateStrategyArgs,
) -> Result<()> {
    ctx.accounts.strategy.active = false;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: DeactivateStrategyArgs)]
pub struct DeactivateStrategy<'info> {
    pub governance: Signer<'info>,

    #[account(has_one = governance, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(mut, seeds = [STRATEGY_SEED.as_bytes(), &args.vault_id.to_le_bytes(), &args.strategy_id.to_le_bytes()], bump = strategy.bump)]
    pub strategy: Account<'info, StrategyState>,
}
