use crate::constants::{TOKEN_PRICE};

pub fn calculate_purchase_price(amount: u64) -> u64 {
    (amount as f64 * TOKEN_PRICE) as u64
}