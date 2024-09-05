use anchor_lang::prelude::*;

#[account]
pub struct MintStat {
    pub start: i64,
    pub end: i64,
    pub total_amount: u64,
    pub amount_left: u64,
    pub profit: u64
}