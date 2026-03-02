use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::Vault;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateFeesArgs {
    pub vault_id: u64,
    pub mint_fee_bps: u16,
    pub withdrawal_fee_bps: u16,
    pub performance_fee_bps: u16,
}

pub fn update_fees(ctx: Context<UpdateFees>, args: UpdateFeesArgs) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.mint_fee_bps = args.mint_fee_bps;
    vault.withdrawal_fee_bps = args.withdrawal_fee_bps;
    vault.performance_fee_bps = args.performance_fee_bps;
    vault.validate_fees()?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: UpdateFeesArgs)]
pub struct UpdateFees<'info> {
    pub governance: Signer<'info>,

    #[account(mut, has_one = governance, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,
}
