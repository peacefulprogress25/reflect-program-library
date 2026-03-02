use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::errors::K1Error;
use crate::state::{StrategyState, Vault};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DeallocateArgs {
    pub vault_id: u64,
    pub strategy_id: u64,
    pub amount: u64,
}

pub fn deallocate(ctx: Context<Deallocate>, args: DeallocateArgs) -> Result<()> {
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

    ctx.accounts.strategy.allocated_assets = ctx
        .accounts
        .strategy
        .allocated_assets
        .checked_sub(args.amount as u128)
        .ok_or(K1Error::MathOverflow)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: DeallocateArgs)]
pub struct Deallocate<'info> {
    pub governance: Signer<'info>,

    #[account(has_one = governance, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(mut, seeds = [STRATEGY_SEED.as_bytes(), &args.vault_id.to_le_bytes(), &args.strategy_id.to_le_bytes()], bump = strategy.bump)]
    pub strategy: Account<'info, StrategyState>,

    #[account(mut, seeds = [VAULT_TREASURY_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump)]
    pub vault_treasury: Account<'info, TokenAccount>,

    #[account(mut, token::mint = usdc_mint)]
    pub strategy_treasury: Account<'info, TokenAccount>,

    /// CHECK: strategy program authority signs transfer back
    pub strategy_authority: Signer<'info>,

    #[account(address = vault.usdc_mint)]
    pub usdc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}
