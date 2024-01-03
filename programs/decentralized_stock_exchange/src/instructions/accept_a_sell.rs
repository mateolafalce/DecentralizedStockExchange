use crate::{state::accounts::*, utils::util::*};
use anchor_lang::{prelude::*, solana_program::*};

pub fn accept_a_sell(ctx: Context<AcceptASell>, amount: u64) -> Result<()> {
    let index: usize = get_index(ctx.accounts.sell_offer.price.clone());
    //validations
    require_keys_eq!(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    );
    require_keys_eq!(ctx.accounts.sell_offer.key(), ctx.accounts.sell_pda.key());
    require_eq!(amount, ctx.accounts.sell_offer.price[index]);

    //lamport transfer
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

    //get &mut accounts
    let system = &mut ctx.accounts.decentralized_exchange_system;
    let stock_account = &mut ctx.accounts.stock_account;
    let seller_account = &mut ctx.accounts.seller_account;
    let buyer_account = &mut ctx.accounts.buyer_account;
    let sell_offer = &mut ctx.accounts.sell_offer;

    //update state
    system.add_historical_exchanges();
    system.sub_total_offers();
    stock_account.sub_current_offers();
    seller_account.sub_participation(sell_offer.sell_or_buy_amount[index]);
    buyer_account.add_participation(sell_offer.sell_or_buy_amount[index]);
    sell_offer.sell_or_buy_amount.remove(index);
    sell_offer.price.remove(index);
    sell_offer.sub_len(PRODUCT);

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
        realloc = 8 + sell_offer.len as usize - PRODUCT as usize,
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
