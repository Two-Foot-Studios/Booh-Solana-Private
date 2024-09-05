use anchor_lang::prelude::*;

use crate::constants::{ADMIN_KEY, SALES_TOKEN_MINT, Errors, TOKEN_MINT, MIN_TOKENS_AMOUNT};
use std::str::FromStr;

pub fn validate_init(
    amount: u64,
    sales_token_key: &Pubkey,
    admin_key: &Pubkey,
    token_key: &Pubkey
) -> Result<()> {
    require!(amount > 0, Errors::IncorrectAmount);
    require!(*admin_key == Pubkey::from_str(ADMIN_KEY).unwrap(), Errors::Forbidden);

    require!(*sales_token_key == Pubkey::from_str(SALES_TOKEN_MINT).unwrap(), Errors::InvalidSalesToken);
    require!(*token_key == Pubkey::from_str(TOKEN_MINT).unwrap(), Errors::InvalidToken);

    Ok(())
}

pub fn validate_mint(
    amount: u64,
    sales_token_key: &Pubkey,
    amount_left: u64
) -> Result<()> {
    require!(amount >= MIN_TOKENS_AMOUNT, Errors::IncorrectAmount);
    require!(*sales_token_key == Pubkey::from_str(SALES_TOKEN_MINT).unwrap(), Errors::InvalidSalesToken);
    require!(amount_left >= amount, Errors::NotEnoughTokensForMint);

    Ok(())
}

pub fn validate_withdraw(
    token_key: &Pubkey
) -> Result<()> {
    require!(*token_key == Pubkey::from_str(TOKEN_MINT).unwrap(), Errors::InvalidToken);

    Ok(())
}

pub fn validate_withdraw_profit(
    sales_token_key: &Pubkey,
    admin_key: &Pubkey
) -> Result<()> {
    require!(*sales_token_key == Pubkey::from_str(SALES_TOKEN_MINT).unwrap(), Errors::InvalidSalesToken);
    require!(*admin_key == Pubkey::from_str(ADMIN_KEY).unwrap(), Errors::Forbidden);

    Ok(())
}