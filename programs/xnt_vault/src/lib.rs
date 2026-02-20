use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMQhgW9J8wz");

#[program]
pub mod xnt_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, strategist: Pubkey) -> Result<()> {
        instructions::initialize::handle(ctx, strategist)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handle(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        instructions::withdraw::handle(ctx, shares)
    }

    pub fn compound(ctx: Context<Compound>, amount: u64) -> Result<()> {
        instructions::compound::handle(ctx, amount)
    }

    pub fn set_pause_flags(ctx: Context<AdminOnly>, pause_flags: u8) -> Result<()> {
        instructions::admin::set_pause_flags(ctx, pause_flags)
    }

    pub fn set_strategist(ctx: Context<AdminOnly>, strategist: Pubkey) -> Result<()> {
        instructions::admin::set_strategist(ctx, strategist)
    }

    pub fn set_caps(
        ctx: Context<AdminOnly>,
        max_total_assets: u64,
        max_deposit_per_tx: u64,
        max_compound_per_tx: u64,
    ) -> Result<()> {
        instructions::admin::set_caps(
            ctx,
            max_total_assets,
            max_deposit_per_tx,
            max_compound_per_tx,
        )
    }
}
