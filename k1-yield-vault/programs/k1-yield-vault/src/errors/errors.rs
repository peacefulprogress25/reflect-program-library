use anchor_lang::prelude::*;

#[error_code]
pub enum K1Error {
    #[msg("Vault paused")]
    VaultPaused,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Invalid fee bps")]
    InvalidFeeBps,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Strategy inactive")]
    StrategyInactive,
    #[msg("Unauthorized reporter")]
    UnauthorizedReporter,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invariant violation")]
    InvariantViolation,
}
