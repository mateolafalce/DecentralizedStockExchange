use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;
use std::collections::BTreeSet;

pub fn sell_offer(
    ctx: Context<SellOffer>,
    sell_amount: u64,
    price: u64
) -> Result<()> {
    // Check if the sell amount is valid
    require!(sell_amount <= ctx.accounts.holder_account.participation, ErrorCode::AmountError);
    let sell_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.sell_offer;
    // Function to check for unique elements in an iterator
    fn unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Ord,
    {
        let mut uniq = BTreeSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }
    let mut copy = sell_offer.price.clone();
    // Check if the price is unique
    require!(unique_elements(vec![copy.push(price)]) == true, ErrorCode::UniquePriceError);
    let (holder_pda, _bump) = Pubkey::find_program_address(&[ctx.accounts.stock_account.key().as_ref(), ctx.accounts.from.key().as_ref()], ctx.program_id);
    // Check if the sell amount is greater than 0
    require!(sell_amount > 0, ErrorCode::AmountError);
    // Check if the stock account PDA key matches the provided stock account key
    require!(ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(), ErrorCode::PubkeyError);
    // Check if the holder PDA key matches the provided holder account key
    require!(holder_pda.key() == ctx.accounts.holder_account.key(), ErrorCode::HolderError);
    let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system;
    let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
    let holder_account: &mut Account<HolderAccount> = &mut ctx.accounts.holder_account;
    // Update the sell offer with the sell amount and price
    sell_offer.sell_or_buy_amount.push(sell_amount);
    sell_offer.price.push(price);
    sell_offer.len += 16;
    // Update the total offers in the system
    system.total_offers += 1;
    // Update the current offers in the stock account
    stock_account.current_offers += 1;
    // Deduct the sell amount from the holder's participation
    holder_account.participation -= sell_amount;
    Ok(())
}

#[derive(Accounts)]
pub struct SellOffer<'info> {
    #[account(mut, seeds = [b"System Account"], bump = decentralized_exchange_system.bump_original)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,
    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,
    #[account(mut, seeds = [stock_account_pda.key().as_ref(), from.key().as_ref()], bump = holder_account.bump_original)]
    pub holder_account: Account<'info, HolderAccount>,
    #[account(
        mut,
        seeds = [b"Sell Account", stock_account_pda.key().as_ref(), from.key().as_ref()],
        bump = sell_offer.bump_original,
        realloc = 8 + sell_offer.len as usize + 16,
        realloc::payer = from,
        realloc::zero = false,
    )]
    pub sell_offer: Account<'info, SellOrBuyAccount>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stock_account_pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}
