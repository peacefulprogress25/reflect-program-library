use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::Vault;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PauseArgs {
    pub vault_id: u64,
}

pub fn pause(ctx: Context<Pause>, _args: PauseArgs) -> Result<()> {
    ctx.accounts.vault.paused = true;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: PauseArgs)]
pub struct Pause<'info> {
    pub governance: Signer<'info>,

    #[account(mut, has_one = governance, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,
}
