use anchor_lang::prelude::*;
#[account]
pub struct UserMintStat {
    pub last_reward: i64,
    pub token_amount: u64
}