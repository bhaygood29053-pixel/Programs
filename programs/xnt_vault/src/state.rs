use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub strategist: Pubkey,
    pub xnt_mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub total_assets: u64,
    pub total_shares: u64,
    pub max_total_assets: u64,
    pub max_deposit_per_tx: u64,
    pub max_compound_per_tx: u64,
    pub pause_flags: u8,
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 8 + (32 * 4) + (8 * 6) + 2;
}

#[account]
pub struct Position {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub shares: u64,
    pub bump: u8,
}

impl Position {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;
}
