use anchor_lang::prelude::*;

use crate::errors::VaultError;

pub fn shares_from_deposit(amount: u64, total_assets: u64, total_shares: u64) -> Result<u64> {
    if amount == 0 {
        return err!(VaultError::ZeroAmount);
    }

    if total_assets == 0 && total_shares == 0 {
        return Ok(amount);
    }

    if total_assets == 0 || total_shares == 0 {
        return err!(VaultError::InvalidShareState);
    }

    let shares = (amount as u128)
        .checked_mul(total_shares as u128)
        .ok_or(VaultError::MathOverflow)?
        .checked_div(total_assets as u128)
        .ok_or(VaultError::MathOverflow)? as u64;

    if shares == 0 {
        return err!(VaultError::InvalidShareState);
    }

    Ok(shares)
}

pub fn assets_from_shares(shares: u64, total_assets: u64, total_shares: u64) -> Result<u64> {
    if shares == 0 {
        return err!(VaultError::ZeroAmount);
    }
    if total_assets == 0 || total_shares == 0 {
        return err!(VaultError::InvalidShareState);
    }

    let assets = (shares as u128)
        .checked_mul(total_assets as u128)
        .ok_or(VaultError::MathOverflow)?
        .checked_div(total_shares as u128)
        .ok_or(VaultError::MathOverflow)? as u64;

    if assets == 0 {
        return err!(VaultError::InvalidShareState);
    }

    Ok(assets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_deposit_is_one_to_one() {
        let shares = shares_from_deposit(100, 0, 0).unwrap();
        assert_eq!(shares, 100);
    }

    #[test]
    fn deposit_after_yield_mints_fewer_shares() {
        let shares = shares_from_deposit(100, 200, 100).unwrap();
        assert_eq!(shares, 50);
    }

    #[test]
    fn withdrawing_shares_gets_proportional_assets() {
        let assets = assets_from_shares(25, 300, 150).unwrap();
        assert_eq!(assets, 50);
    }
}
