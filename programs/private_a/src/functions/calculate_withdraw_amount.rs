use anchor_lang::prelude::*;
use crate::constants::{SECONDS_IN_YEAR};

use crate::structs::{MintStat, UserMintStat};

pub fn calculate_withdraw_amount(user_stat: &UserMintStat, mint_stat: &MintStat) -> u64 {
    if user_stat.last_reward >= mint_stat.end {
        return u64::default();
    }

    let current_time = Clock::get().unwrap().unix_timestamp;
    if current_time <= mint_stat.start {
        return u64::default();
    }

    if user_stat.last_reward == i64::default() {
        return if current_time >= mint_stat.end {
            user_stat.token_amount
        } else {
            calculate_amount(user_stat.token_amount, current_time - mint_stat.start)
        }
    }

    return if current_time >= mint_stat.end {
        calculate_amount(user_stat.token_amount, mint_stat.end - user_stat.last_reward)
    } else {
        calculate_amount(user_stat.token_amount, current_time - user_stat.last_reward)
    };
}

fn calculate_amount(full_amount: u64, time_passed: i64) -> u64 {
    let part_of_full: f64 = time_passed as f64 / SECONDS_IN_YEAR as f64;
    (full_amount as f64 * part_of_full) as u64
}