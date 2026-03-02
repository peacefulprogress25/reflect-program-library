use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::*;
use crate::state::Vault;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeVaultArgs {
    pub vault_id: u64,
    pub mint_fee_bps: u16,
    pub withdrawal_fee_bps: u16,
    pub performance_fee_bps: u16,
}

pub fn initialize_vault(ctx: Context<InitializeVault>, args: InitializeVaultArgs) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.usdc_mint = ctx.accounts.usdc_mint.key();
    vault.share_mint = ctx.accounts.share_mint.key();
    vault.treasury = ctx.accounts.treasury_fee_account.key();
    vault.governance = ctx.accounts.governance.key();
    vault.reporter = ctx.accounts.reporter.key();
    vault.total_shares = 0;
    vault.offchain_nav = 0;
    vault.strategy_value_sum = 0;
    vault.high_water_mark = 0;
    vault.last_total_assets = 0;
    vault.mint_fee_bps = args.mint_fee_bps;
    vault.withdrawal_fee_bps = args.withdrawal_fee_bps;
    vault.performance_fee_bps = args.performance_fee_bps;
    vault.paused = false;
    vault.strategy_count = 0;
    vault.bump = ctx.bumps.vault;
    vault.validate_fees()?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: InitializeVaultArgs)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub governance: Signer<'info>,

    /// CHECK: optional reporter authority
    pub reporter: UncheckedAccount<'info>,

    #[account(
        init,
        payer = governance,
        space = 8 + Vault::INIT_SPACE,
        seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = governance,
        token::mint = usdc_mint,
        token::authority = vault,
        seeds = [VAULT_TREASURY_SEED.as_bytes(), &args.vault_id.to_le_bytes()],
        bump
    )]
    pub vault_treasury: Account<'info, TokenAccount>,

    #[account(
        seeds = [SHARE_MINT_AUTH_SEED.as_bytes(), &args.vault_id.to_le_bytes()],
        bump
    )]
    /// CHECK: PDA mint authority
    pub share_mint_authority: UncheckedAccount<'info>,

    #[account(constraint = usdc_mint.decimals == ASSET_DECIMALS)]
    pub usdc_mint: Account<'info, Mint>,

    #[account(mut, constraint = share_mint.mint_authority.unwrap() == share_mint_authority.key())]
    pub share_mint: Account<'info, Mint>,

    #[account(mut)]
    pub treasury_fee_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
