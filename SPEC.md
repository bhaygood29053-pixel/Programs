# XNT Auto-Compound Vault Specification

## Scope
- Single-asset vault for XNT only.
- Users deposit XNT and receive internal accounting shares.
- Strategist compounds rewards in XNT into the same vault.
- Withdrawals redeem shares for proportional XNT.

## Explicit non-goals
- No multi-asset deposits.
- No price feed logic.

## Roles
- **Admin**: initializes vault, updates strategist, pause flags, and caps.
- **Strategist**: may call `compound`.
- **User**: may `deposit` and `withdraw` if unpaused.

## Pause flags
- `PAUSE_DEPOSIT`
- `PAUSE_WITHDRAW`
- `PAUSE_COMPOUND`

## Share math
- First deposit: `shares = assets`.
- Subsequent deposit: `minted_shares = amount * total_shares / total_assets`.
- Withdrawal assets out: `assets_out = shares * total_assets / total_shares`.

## Caps
- Max total assets.
- Max deposit per transaction.
- Max compound per transaction.

## Program layout
- `instructions/`
- `state.rs`
- `math.rs`
- `errors.rs`
- `events.rs`
