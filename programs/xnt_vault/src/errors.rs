use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid share state")]
    InvalidShareState,
    #[msg("Deposits are paused")]
    DepositsPaused,
    #[msg("Withdrawals are paused")]
    WithdrawalsPaused,
    #[msg("Compounding is paused")]
    CompoundPaused,
    #[msg("Unauthorized caller")]
    Unauthorized,
    #[msg("Vault total asset cap exceeded")]
    TotalAssetCapExceeded,
    #[msg("Deposit amount exceeds per-transaction cap")]
    DepositCapExceeded,
    #[msg("Compound amount exceeds per-transaction cap")]
    CompoundCapExceeded,
    #[msg("Insufficient user shares")]
    InsufficientShares,
    #[msg("Invalid pause flags")]
    InvalidPauseFlags,
}
