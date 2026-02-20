use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::{
    constants::{PAUSE_DEPOSIT, POSITION_SEED, VAULT_AUTHORITY_SEED},
    errors::VaultError,
    events::DepositEvent,
    math::shares_from_deposit,
    state::{Position, Vault},
};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = vault.xnt_mint == owner_token_account.mint,
        constraint = vault.vault_token_account == vault_token_account.key(),
    )]
    pub vault: Account<'info, Vault>,

    /// CHECK: PDA authority used for vault token ATA ownership.
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, vault.key().as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        space = Position::LEN,
        seeds = [POSITION_SEED, vault.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(amount > 0, VaultError::ZeroAmount);
    require!(
        vault.pause_flags & PAUSE_DEPOSIT == 0,
        VaultError::DepositsPaused
    );
    require!(
        amount <= vault.max_deposit_per_tx,
        VaultError::DepositCapExceeded
    );

    let new_total_assets = vault
        .total_assets
        .checked_add(amount)
        .ok_or(VaultError::MathOverflow)?;
    require!(
        new_total_assets <= vault.max_total_assets,
        VaultError::TotalAssetCapExceeded
    );

    let minted_shares = shares_from_deposit(amount, vault.total_assets, vault.total_shares)?;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    let position = &mut ctx.accounts.position;
    if position.owner == Pubkey::default() {
        position.owner = ctx.accounts.owner.key();
        position.vault = vault.key();
        position.bump = ctx.bumps.position;
    }

    position.shares = position
        .shares
        .checked_add(minted_shares)
        .ok_or(VaultError::MathOverflow)?;
    vault.total_assets = new_total_assets;
    vault.total_shares = vault
        .total_shares
        .checked_add(minted_shares)
        .ok_or(VaultError::MathOverflow)?;

    emit!(DepositEvent {
        owner: ctx.accounts.owner.key(),
        amount,
        minted_shares,
        total_assets: vault.total_assets,
        total_shares: vault.total_shares,
    });

    Ok(())
}
