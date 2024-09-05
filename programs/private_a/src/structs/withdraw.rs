use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::structs::{MintStat, UserMintStat};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub mint_of_token: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint = user_token_ata.owner == user.key(),
        constraint = user_token_ata.mint == mint_of_token.key()
    )]
    pub user_token_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [user.key().as_ref()],
        bump,
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
        seeds = [b"mint_bank"],
        bump,
        token::mint = mint_of_token,
        token::authority = mint_bank
    )]
    pub mint_bank: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}