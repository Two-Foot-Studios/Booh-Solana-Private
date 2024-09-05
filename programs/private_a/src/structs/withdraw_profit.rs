use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::structs::{MintStat};
#[derive(Accounts)]
pub struct WithdrawProfit<'info> {
    pub sales_mint_of_token: Account<'info, Mint>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint_stat"],
        bump,
    )]
    pub mint_stat: Account<'info, MintStat>,

    #[account(
        mut,
        seeds = [b"sales_mint_bank"],
        bump,
        token::mint = sales_mint_of_token,
        token::authority = sales_mint_bank
    )]
    pub sales_mint_bank: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin_sales_token_ata.owner == admin.key(),
        constraint = admin_sales_token_ata.mint == sales_mint_of_token.key()
    )]
    pub admin_sales_token_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}