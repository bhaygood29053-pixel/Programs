use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    constants::{
        DEFAULT_MAX_COMPOUND_PER_TX, DEFAULT_MAX_DEPOSIT_PER_TX, DEFAULT_MAX_TOTAL_ASSETS,
        VAULT_AUTHORITY_SEED, VAULT_SEED,
    },
    state::Vault,
};

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: PDA authority for vault-owned token account.
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, vault.key().as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        space = Vault::LEN,
        seeds = [VAULT_SEED, xnt_mint.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    pub xnt_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = xnt_mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<InitializeVault>, strategist: Pubkey) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.admin = ctx.accounts.admin.key();
    vault.strategist = strategist;
    vault.xnt_mint = ctx.accounts.xnt_mint.key();
    vault.vault_token_account = ctx.accounts.vault_token_account.key();
    vault.total_assets = 0;
    vault.total_shares = 0;
    vault.max_total_assets = DEFAULT_MAX_TOTAL_ASSETS;
    vault.max_deposit_per_tx = DEFAULT_MAX_DEPOSIT_PER_TX;
    vault.max_compound_per_tx = DEFAULT_MAX_COMPOUND_PER_TX;
    vault.pause_flags = 0;
    vault.bump = ctx.bumps.vault;
    Ok(())
}
