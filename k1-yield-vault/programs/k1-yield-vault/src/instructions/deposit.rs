use std::convert::TryInto;

use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::errors::K1Error;
use crate::state::Vault;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositArgs {
    pub vault_id: u64,
    pub amount: u64,
}

pub fn deposit(ctx: Context<Deposit>, args: DepositArgs) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.check_not_paused()?;

    let total_assets_before = vault.total_assets(ctx.accounts.vault_treasury.amount)?;
    let shares_to_mint = vault.compute_deposit_shares(args.amount, total_assets_before)?;
    let (user_shares, fee_shares) = vault.apply_mint_fee(shares_to_mint)?;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_usdc_account.to_account_info(),
                to: ctx.accounts.vault_treasury.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        args.amount,
    )?;

    let signer_seeds: &[&[u8]] = &[
        SHARE_MINT_AUTH_SEED.as_bytes(),
        &args.vault_id.to_le_bytes(),
        &[ctx.bumps.share_mint_authority],
    ];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.share_mint.to_account_info(),
                to: ctx.accounts.user_share_account.to_account_info(),
                authority: ctx.accounts.share_mint_authority.to_account_info(),
            },
            &[signer_seeds],
        ),
        user_shares.try_into().map_err(|_| K1Error::MathOverflow)?,
    )?;

    if fee_shares > 0 {
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.share_mint.to_account_info(),
                    to: ctx.accounts.treasury_share_account.to_account_info(),
                    authority: ctx.accounts.share_mint_authority.to_account_info(),
                },
                &[signer_seeds],
            ),
            fee_shares.try_into().map_err(|_| K1Error::MathOverflow)?,
        )?;
    }

    vault.total_shares = vault
        .total_shares
        .checked_add(shares_to_mint)
        .ok_or(K1Error::MathOverflow)?;

    if vault.high_water_mark == 0 {
        vault.high_water_mark = total_assets_before
            .checked_add(args.amount as u128)
            .ok_or(K1Error::MathOverflow)?;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: DepositArgs)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(mut, seeds = [VAULT_TREASURY_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump)]
    pub vault_treasury: Account<'info, TokenAccount>,

    #[account(seeds = [SHARE_MINT_AUTH_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump)]
    /// CHECK: PDA authority
    pub share_mint_authority: UncheckedAccount<'info>,

    #[account(mut, address = vault.usdc_mint)]
    pub usdc_mint: Account<'info, Mint>,

    #[account(mut, address = vault.share_mint)]
    pub share_mint: Account<'info, Mint>,

    #[account(mut, token::mint = usdc_mint, token::authority = user)]
    pub user_usdc_account: Account<'info, TokenAccount>,

    #[account(mut, token::mint = share_mint, token::authority = user)]
    pub user_share_account: Account<'info, TokenAccount>,

    #[account(mut, token::mint = share_mint, address = vault.treasury)]
    pub treasury_share_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
