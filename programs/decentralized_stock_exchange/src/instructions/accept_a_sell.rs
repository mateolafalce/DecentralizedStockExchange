use crate::{errors::ErrorCode, state::accounts::*, validations::*};
use anchor_lang::{
    prelude::*, solana_program::account_info::AccountInfo, solana_program::pubkey::Pubkey,
    solana_program::system_instruction,
};

pub fn accept_a_sell(ctx: Context<AcceptASell>, price: u64) -> Result<()> {
    /*validations*/
    equal_accounts(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    )
    .unwrap();
    equal_accounts(ctx.accounts.sell_offer.key(), ctx.accounts.sell_pda.key()).unwrap();
    // Find the index of the specified price in the sell offer prices array
    let index = ctx
        .accounts
        .sell_offer
        .price
        .iter()
        .position(|&price| price == price)
        .unwrap();
    // Check if the specified price matches the price at the found index
    require!(
        price == ctx.accounts.sell_offer.price[index],
        ErrorCode::PriceError
    );
    // Invoke the system program to transfer funds from "from" to the sell offer account
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.sell_offer.key(),
            ctx.accounts.sell_offer.price[index],
        ),
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.sell_pda.to_account_info().clone(),
        ],
    )
    .expect("Error");
    let system: &mut Account<SystemExchangeAccount> =
        &mut ctx.accounts.decentralized_exchange_system;
    let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
    let seller_account: &mut Account<HolderAccount> = &mut ctx.accounts.seller_account;
    let buyer_account: &mut Account<HolderAccount> = &mut ctx.accounts.buyer_account;
    let sell_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.sell_offer;
    // Increment the historical exchanges counter
    system.historical_exchanges += 1;
    // Decrement the total offers counter
    system.total_offers -= 1;
    // Decrement the current offers counter in the stock account
    stock_account.current_offers -= 1;
    // Subtract the sell or buy amount from the seller's participation
    seller_account.participation -= sell_offer.sell_or_buy_amount[index];
    // Add the sell or buy amount to the buyer's participation
    buyer_account.participation += sell_offer.sell_or_buy_amount[index];
    // Remove the sell or buy amount at the specified index
    sell_offer.sell_or_buy_amount.remove(index);
    // Remove the price at the specified index
    sell_offer.price.remove(index);
    // Decrement the length of the sell offer by 16
    sell_offer.len -= 16;
    Ok(())
}

#[derive(Accounts)]
pub struct AcceptASell<'info> {
    #[account(mut, seeds = [b"System Account"], bump = decentralized_exchange_system.bump_original)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,
    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,
    #[account(mut, seeds = [stock_account_pda.key().as_ref(), from.key().as_ref()], bump = seller_account.bump_original)]
    pub seller_account: Account<'info, HolderAccount>,
    #[account(mut, seeds = [stock_account_pda.key().as_ref(), from.key().as_ref()], bump = buyer_account.bump_original)]
    pub buyer_account: Account<'info, HolderAccount>,
    #[account(
        mut,
        seeds = [b"Sell Account", stock_account_pda.key().as_ref(), from.key().as_ref()],
        bump = sell_offer.bump_original,
        realloc = 8 + sell_offer.len as usize - 16,
        realloc::payer = from,
        realloc::zero = false,
    )]
    pub sell_offer: Account<'info, SellOrBuyAccount>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stock_account_pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub sell_pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
