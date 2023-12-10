use anchor_lang::prelude::{AccountInfo, ProgramError};
use std::collections::BTreeSet;

pub const PRODUCT: u64 = 8 + 8; // price + amount (u64 each)
pub const NAME: u64 = 50;
pub const DESCRIPTION: u64 = 200;
pub const BUY_ACCOUNT: u64 = 65; // look accounts.rs comments
pub const SELL_ACCOUNT: u64 = 65; // look accounts.rs comments

pub fn get_index(price: Vec<u64>) -> usize {
    price.iter().position(|&price| price == price).unwrap()
}

pub fn pda_transfer(from: AccountInfo, to: AccountInfo, amount: u64) -> Result<(), ProgramError> {
    **from.try_borrow_mut_lamports().unwrap() -= amount;
    **to.try_borrow_mut_lamports().unwrap() += amount;
    Ok(())
}

pub fn unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Ord,
{
    let mut uniq = BTreeSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
