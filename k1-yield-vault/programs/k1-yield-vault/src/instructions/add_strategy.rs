use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::{StrategyState, Vault};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddStrategyArgs {
    pub vault_id: u64,
    pub strategy_id: u64,
}

pub fn add_strategy(ctx: Context<AddStrategy>, _args: AddStrategyArgs) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let strategy = &mut ctx.accounts.strategy;

    strategy.strategy_program = ctx.accounts.strategy_program.key();
    strategy.strategy_treasury = ctx.accounts.strategy_treasury.key();
    strategy.allocated_assets = 0;
    strategy.last_reported_value = 0;
    strategy.active = true;
    strategy.bump = ctx.bumps.strategy;

    vault.strategy_count = vault.strategy_count.saturating_add(1);
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: AddStrategyArgs)]
pub struct AddStrategy<'info> {
    pub governance: Signer<'info>,

    #[account(mut, has_one = governance, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = governance,
        space = 8 + StrategyState::INIT_SPACE,
        seeds = [STRATEGY_SEED.as_bytes(), &args.vault_id.to_le_bytes(), &args.strategy_id.to_le_bytes()],
        bump
    )]
    pub strategy: Account<'info, StrategyState>,

    /// CHECK: external strategy program id
    pub strategy_program: UncheckedAccount<'info>,

    /// CHECK: external strategy treasury account
    pub strategy_treasury: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
