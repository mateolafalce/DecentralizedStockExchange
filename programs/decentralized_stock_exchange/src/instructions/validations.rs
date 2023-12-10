use crate::{errors::ErrorCode, utils::utils::unique_elements};
use anchor_lang::prelude::*;

pub fn equal_accounts(a: Pubkey, b: Pubkey) -> Result<()> {
    require!(a == b, ErrorCode::PubkeyError);
    Ok(())
}

pub fn equal_price(a: u64, b: u64) -> Result<()> {
    require!(a == b, ErrorCode::PriceError);
    Ok(())
}

pub fn greater_than_0(buy_amount: u64) -> Result<()> {
    require!(buy_amount > 0, ErrorCode::AmountError);
    Ok(())
}

pub fn less_or_equal_than(a: u64, b: u64) -> Result<()> {
    require!(a <= b, ErrorCode::SizeError);
    Ok(())
}

pub fn check_current_time(date_to_go_public: i64) -> Result<()> {
    require!(
        date_to_go_public > Clock::get().unwrap().unix_timestamp,
        ErrorCode::Date
    );
    Ok(())
}

pub fn check_unique_of_price(prices: Vec<u64>) -> Result<()> {
    require!(
        unique_elements(vec![prices]) == true,
        ErrorCode::UniquePriceError
    );
    Ok(())
}
