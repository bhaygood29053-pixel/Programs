use anchor_lang::prelude::*;

#[event]
pub struct DepositEvent {
    pub owner: Pubkey,
    pub amount: u64,
    pub minted_shares: u64,
    pub total_assets: u64,
    pub total_shares: u64,
}

#[event]
pub struct WithdrawEvent {
    pub owner: Pubkey,
    pub shares_burned: u64,
    pub assets_out: u64,
    pub total_assets: u64,
    pub total_shares: u64,
}

#[event]
pub struct CompoundEvent {
    pub strategist: Pubkey,
    pub amount: u64,
    pub total_assets: u64,
    pub total_shares: u64,
}

#[event]
pub struct PauseFlagsUpdatedEvent {
    pub admin: Pubkey,
    pub pause_flags: u8,
}

#[event]
pub struct StrategistUpdatedEvent {
    pub admin: Pubkey,
    pub strategist: Pubkey,
}

#[event]
pub struct CapsUpdatedEvent {
    pub admin: Pubkey,
    pub max_total_assets: u64,
    pub max_deposit_per_tx: u64,
    pub max_compound_per_tx: u64,
}
