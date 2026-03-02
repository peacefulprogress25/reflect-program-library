use std::convert::TryInto;

use anchor_lang::prelude::*;
use anchor_spl::token::{
    burn, mint_to, transfer, Burn, Mint, MintTo, Token, TokenAccount, Transfer,
};

use crate::constants::*;
use crate::errors::K1Error;
use crate::state::Vault;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawArgs {
    pub vault_id: u64,
    pub shares_to_burn: u64,
}

pub fn withdraw(ctx: Context<Withdraw>, args: WithdrawArgs) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.check_not_paused()?;

    let total_assets = vault.total_assets(ctx.accounts.vault_treasury.amount)?;
    let assets_out = vault.compute_withdraw_assets(args.shares_to_burn, total_assets)?;
    let (user_assets, fee_assets) = vault.apply_withdraw_fee(assets_out)?;

    require!(
        (ctx.accounts.vault_treasury.amount as u128) >= user_assets,
        K1Error::InsufficientLiquidity
    );

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.share_mint.to_account_info(),
                from: ctx.accounts.user_share_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        args.shares_to_burn,
    )?;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_treasury.to_account_info(),
                to: ctx.accounts.user_usdc_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            &[&[
                VAULT_SEED.as_bytes(),
                &args.vault_id.to_le_bytes(),
                &[vault.bump],
            ]],
        ),
        user_assets.try_into().map_err(|_| K1Error::MathOverflow)?,
    )?;

    let fee_shares = if fee_assets > 0 {
        vault.fee_assets_to_shares(fee_assets, total_assets, assets_out)?
    } else {
        0
    };

    if fee_shares > 0 {
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.share_mint.to_account_info(),
                    to: ctx.accounts.treasury_share_account.to_account_info(),
                    authority: ctx.accounts.share_mint_authority.to_account_info(),
                },
                &[&[
                    SHARE_MINT_AUTH_SEED.as_bytes(),
                    &args.vault_id.to_le_bytes(),
                    &[ctx.bumps.share_mint_authority],
                ]],
            ),
            fee_shares.try_into().map_err(|_| K1Error::MathOverflow)?,
        )?;
    }

    vault.total_shares = vault
        .total_shares
        .checked_sub(args.shares_to_burn as u128)
        .ok_or(K1Error::MathOverflow)?
        .checked_add(fee_shares)
        .ok_or(K1Error::MathOverflow)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: WithdrawArgs)]
pub struct Withdraw<'info> {
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
