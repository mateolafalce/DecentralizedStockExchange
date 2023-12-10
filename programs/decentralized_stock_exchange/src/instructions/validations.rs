use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

pub fn equal_accounts(a: Pubkey, b: Pubkey) -> Result<()> {
    require!(a == b, ErrorCode::PubkeyError);
    Ok(())
}

pub fn equal_price(a: u64, b: u64) -> Result<()> {
    require!(a == b, ErrorCode::PriceError);
    Ok(())
}
