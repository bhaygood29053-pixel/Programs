pub const VAULT_SEED: &[u8] = b"vault";
pub const VAULT_AUTHORITY_SEED: &[u8] = b"vault_authority";
pub const POSITION_SEED: &[u8] = b"position";

pub const PAUSE_DEPOSIT: u8 = 1 << 0;
pub const PAUSE_WITHDRAW: u8 = 1 << 1;
pub const PAUSE_COMPOUND: u8 = 1 << 2;

pub const DEFAULT_MAX_TOTAL_ASSETS: u64 = 1_000_000_000_000;
pub const DEFAULT_MAX_DEPOSIT_PER_TX: u64 = 100_000_000_000;
pub const DEFAULT_MAX_COMPOUND_PER_TX: u64 = 100_000_000_000;
