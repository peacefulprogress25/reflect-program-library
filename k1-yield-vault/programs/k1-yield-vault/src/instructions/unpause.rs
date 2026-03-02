use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::Vault;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UnpauseArgs {
    pub vault_id: u64,
}

pub fn unpause(ctx: Context<Unpause>, _args: UnpauseArgs) -> Result<()> {
    ctx.accounts.vault.paused = false;
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: UnpauseArgs)]
pub struct Unpause<'info> {
    pub governance: Signer<'info>,

    #[account(mut, has_one = governance, seeds = [VAULT_SEED.as_bytes(), &args.vault_id.to_le_bytes()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,
}
