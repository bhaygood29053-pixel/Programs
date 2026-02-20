use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::{
    constants::{PAUSE_WITHDRAW, POSITION_SEED, VAULT_AUTHORITY_SEED},
    errors::VaultError,
    events::WithdrawEvent,
    math::assets_from_shares,
    state::{Position, Vault},
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// CHECK: PDA authority for vault token transfers.
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, vault.key().as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [POSITION_SEED, vault.key().as_ref(), owner.key().as_ref()],
        bump = position.bump,
        constraint = position.owner == owner.key(),
        constraint = position.vault == vault.key(),
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault.vault_token_account == vault_token_account.key(),
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(shares > 0, VaultError::ZeroAmount);
    require!(
        vault.pause_flags & PAUSE_WITHDRAW == 0,
        VaultError::WithdrawalsPaused
    );

    let position = &mut ctx.accounts.position;
    require!(position.shares >= shares, VaultError::InsufficientShares);

    let assets_out = assets_from_shares(shares, vault.total_assets, vault.total_shares)?;

    let signer_seeds: &[&[&[u8]]] = &[&[
        VAULT_AUTHORITY_SEED,
        vault.key().as_ref(),
        &[ctx.bumps.vault_authority],
    ]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.owner_token_account.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds,
        ),
        assets_out,
    )?;

    position.shares = position
        .shares
        .checked_sub(shares)
        .ok_or(VaultError::MathOverflow)?;
    vault.total_shares = vault
        .total_shares
        .checked_sub(shares)
        .ok_or(VaultError::MathOverflow)?;
    vault.total_assets = vault
        .total_assets
        .checked_sub(assets_out)
        .ok_or(VaultError::MathOverflow)?;

    emit!(WithdrawEvent {
        owner: ctx.accounts.owner.key(),
        shares_burned: shares,
        assets_out,
        total_assets: vault.total_assets,
        total_shares: vault.total_shares,
    });

    Ok(())
}
