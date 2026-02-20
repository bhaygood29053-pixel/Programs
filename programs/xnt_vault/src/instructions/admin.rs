use anchor_lang::prelude::*;

use crate::{
    constants::{PAUSE_COMPOUND, PAUSE_DEPOSIT, PAUSE_WITHDRAW},
    errors::VaultError,
    events::{CapsUpdatedEvent, PauseFlagsUpdatedEvent, StrategistUpdatedEvent},
    state::Vault,
};

#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        constraint = vault.admin == admin.key() @ VaultError::Unauthorized,
    )]
    pub vault: Account<'info, Vault>,
}

pub fn set_pause_flags(ctx: Context<AdminOnly>, pause_flags: u8) -> Result<()> {
    let allowed_flags = PAUSE_DEPOSIT | PAUSE_WITHDRAW | PAUSE_COMPOUND;
    require!(
        pause_flags & !allowed_flags == 0,
        VaultError::InvalidPauseFlags
    );

    ctx.accounts.vault.pause_flags = pause_flags;

    emit!(PauseFlagsUpdatedEvent {
        admin: ctx.accounts.admin.key(),
        pause_flags,
    });
    Ok(())
}

pub fn set_strategist(ctx: Context<AdminOnly>, strategist: Pubkey) -> Result<()> {
    ctx.accounts.vault.strategist = strategist;

    emit!(StrategistUpdatedEvent {
        admin: ctx.accounts.admin.key(),
        strategist,
    });
    Ok(())
}

pub fn set_caps(
    ctx: Context<AdminOnly>,
    max_total_assets: u64,
    max_deposit_per_tx: u64,
    max_compound_per_tx: u64,
) -> Result<()> {
    require!(max_total_assets > 0, VaultError::ZeroAmount);
    require!(max_deposit_per_tx > 0, VaultError::ZeroAmount);
    require!(max_compound_per_tx > 0, VaultError::ZeroAmount);

    let vault = &mut ctx.accounts.vault;
    vault.max_total_assets = max_total_assets;
    vault.max_deposit_per_tx = max_deposit_per_tx;
    vault.max_compound_per_tx = max_compound_per_tx;

    emit!(CapsUpdatedEvent {
        admin: ctx.accounts.admin.key(),
        max_total_assets,
        max_deposit_per_tx,
        max_compound_per_tx,
    });

    Ok(())
}
