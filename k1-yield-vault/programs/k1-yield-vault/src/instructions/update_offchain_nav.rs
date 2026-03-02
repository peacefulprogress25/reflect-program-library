use std::convert::TryInto;

use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

use crate::constants::*;
use crate::errors::K1Error;
use crate::state::Vault;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateOffchainNavArgs {
    pub vault_id: u64,
    pub value: u128,
}

pub fn update_offchain_nav(
    ctx: Context<UpdateOffchainNav>,
    args: UpdateOffchainNavArgs,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.offchain_nav = args.value;

    let total_assets = vault.total_assets(ctx.accounts.vault_treasury.amount)?;
    let fee_shares = vault.apply_performance_fee(total_assets)?;

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

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: UpdateOffchainNavArgs)]
pub struct UpdateOffchainNav<'info> {
    pub governance: Signer<'info>,

    #[account(mut, has_one = governance, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(mut, seeds = [VAULT_TREASURY_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump)]
    pub vault_treasury: Account<'info, TokenAccount>,

    #[account(seeds = [SHARE_MINT_AUTH_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump)]
    /// CHECK: PDA authority
    pub share_mint_authority: UncheckedAccount<'info>,

    #[account(mut, address = vault.share_mint)]
    pub share_mint: Account<'info, Mint>,

    #[account(mut, token::mint = share_mint, address = vault.treasury)]
    pub treasury_share_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
