use anchor_lang::prelude::{AccountInfo, ProgramError};

pub fn get_index(price: Vec<u64>) -> usize {
    price.iter().position(|&price| price == price).unwrap()
}

pub fn pda_transfer(from: AccountInfo, to: AccountInfo, amount: u64) -> Result<(), ProgramError> {
    **from.try_borrow_mut_lamports().unwrap() -= amount;
    **to.try_borrow_mut_lamports().unwrap() += amount;
    Ok(())
}
