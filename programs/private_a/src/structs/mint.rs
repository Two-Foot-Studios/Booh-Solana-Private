use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::structs::{MintStat, UserMintStat};

#[derive(Accounts)]
pub struct Mint<'info> {
    pub sales_mint_of_token: Account<'info, anchor_spl::token::Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 8 + 8
    )]
    pub user_mint_stat: Account<'info, UserMintStat>,

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
        constraint = user_sales_token_ata.owner == user.key(),
        constraint = user_sales_token_ata.mint == sales_mint_of_token.key()
    )]
    pub user_sales_token_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}