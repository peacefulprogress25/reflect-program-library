use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("K1VaULt111111111111111111111111111111111111");

#[program]
pub mod k1_yield_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        args: InitializeVaultArgs,
    ) -> Result<()> {
        instructions::initialize_vault(ctx, args)
    }

    pub fn deposit(ctx: Context<Deposit>, args: DepositArgs) -> Result<()> {
        instructions::deposit(ctx, args)
    }

    pub fn withdraw(ctx: Context<Withdraw>, args: WithdrawArgs) -> Result<()> {
        instructions::withdraw(ctx, args)
    }

    pub fn add_strategy(ctx: Context<AddStrategy>, args: AddStrategyArgs) -> Result<()> {
        instructions::add_strategy(ctx, args)
    }

    pub fn deactivate_strategy(
        ctx: Context<DeactivateStrategy>,
        args: DeactivateStrategyArgs,
    ) -> Result<()> {
        instructions::deactivate_strategy(ctx, args)
    }

    pub fn allocate(ctx: Context<Allocate>, args: AllocateArgs) -> Result<()> {
        instructions::allocate(ctx, args)
    }

    pub fn deallocate(ctx: Context<Deallocate>, args: DeallocateArgs) -> Result<()> {
        instructions::deallocate(ctx, args)
    }

    pub fn report_strategy(ctx: Context<ReportStrategy>, args: ReportStrategyArgs) -> Result<()> {
        instructions::report_strategy(ctx, args)
    }

    pub fn update_offchain_nav(
        ctx: Context<UpdateOffchainNav>,
        args: UpdateOffchainNavArgs,
    ) -> Result<()> {
        instructions::update_offchain_nav(ctx, args)
    }

    pub fn pause(ctx: Context<Pause>, args: PauseArgs) -> Result<()> {
        instructions::pause(ctx, args)
    }

    pub fn unpause(ctx: Context<Unpause>, args: UnpauseArgs) -> Result<()> {
        instructions::unpause(ctx, args)
    }

    pub fn emergency_withdraw_strategy(
        ctx: Context<EmergencyWithdrawStrategy>,
        args: EmergencyWithdrawStrategyArgs,
    ) -> Result<()> {
        instructions::emergency_withdraw_strategy(ctx, args)
    }

    pub fn update_fees(ctx: Context<UpdateFees>, args: UpdateFeesArgs) -> Result<()> {
        instructions::update_fees(ctx, args)
    }
}
