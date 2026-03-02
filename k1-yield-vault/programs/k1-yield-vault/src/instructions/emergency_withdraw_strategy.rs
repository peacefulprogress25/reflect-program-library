use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::errors::K1Error;
use crate::state::{StrategyState, Vault};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct EmergencyWithdrawStrategyArgs {
    pub vault_id: u64,
    pub strategy_id: u64,
    pub amount: u64,
}

pub fn emergency_withdraw_strategy(
    ctx: Context<EmergencyWithdrawStrategy>,
    args: EmergencyWithdrawStrategyArgs,
) -> Result<()> {
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.strategy_treasury.to_account_info(),
                to: ctx.accounts.vault_treasury.to_account_info(),
                authority: ctx.accounts.strategy_authority.to_account_info(),
            },
        ),
        args.amount,
    )?;

    let strategy = &mut ctx.accounts.strategy;
    strategy.active = false;
    strategy.allocated_assets = 0;

    ctx.accounts.vault.strategy_value_sum = ctx
        .accounts
        .vault
        .strategy_value_sum
        .checked_sub(strategy.last_reported_value)
        .ok_or(K1Error::MathOverflow)?;
    strategy.last_reported_value = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: EmergencyWithdrawStrategyArgs)]
pub struct EmergencyWithdrawStrategy<'info> {
    pub governance: Signer<'info>,

    #[account(mut, has_one = governance, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(mut, seeds = [STRATEGY_SEED.as_bytes(), &args.vault_id.to_le_bytes(), &args.strategy_id.to_le_bytes()], bump = strategy.bump)]
    pub strategy: Account<'info, StrategyState>,

    #[account(mut, seeds = [VAULT_TREASURY_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump)]
    pub vault_treasury: Account<'info, TokenAccount>,

    #[account(mut, token::mint = usdc_mint)]
    pub strategy_treasury: Account<'info, TokenAccount>,

    pub strategy_authority: Signer<'info>,

    #[account(address = vault.usdc_mint)]
    pub usdc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}
