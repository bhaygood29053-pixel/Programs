use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::{constants::PAUSE_COMPOUND, errors::VaultError, events::CompoundEvent, state::Vault};

#[derive(Accounts)]
pub struct Compound<'info> {
    #[account(mut)]
    pub strategist: Signer<'info>,

    #[account(
        mut,
        constraint = vault.strategist == strategist.key() @ VaultError::Unauthorized,
        constraint = vault.vault_token_account == vault_token_account.key(),
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut, constraint = strategist_reward_token_account.mint == vault.xnt_mint)]
    pub strategist_reward_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<Compound>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(amount > 0, VaultError::ZeroAmount);
    require!(
        vault.pause_flags & PAUSE_COMPOUND == 0,
        VaultError::CompoundPaused
    );
    require!(
        amount <= vault.max_compound_per_tx,
        VaultError::CompoundCapExceeded
    );

    let new_total_assets = vault
        .total_assets
        .checked_add(amount)
        .ok_or(VaultError::MathOverflow)?;
    require!(
        new_total_assets <= vault.max_total_assets,
        VaultError::TotalAssetCapExceeded
    );

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx
                    .accounts
                    .strategist_reward_token_account
                    .to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.strategist.to_account_info(),
            },
        ),
        amount,
    )?;

    vault.total_assets = new_total_assets;

    emit!(CompoundEvent {
        strategist: ctx.accounts.strategist.key(),
        amount,
        total_assets: vault.total_assets,
        total_shares: vault.total_shares,
    });

    Ok(())
}
