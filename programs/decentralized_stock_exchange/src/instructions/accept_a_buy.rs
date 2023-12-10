use crate::{
    state::accounts::*,
    utils::utils::{get_index, pda_transfer, PRODUCT},
    validations::*,
};
use anchor_lang::{
    prelude::*, solana_program::account_info::AccountInfo, solana_program::pubkey::Pubkey,
};

pub fn accept_a_buy(ctx: Context<AcceptABuy>, amount: u64) -> Result<()> {
    let index: usize = get_index(ctx.accounts.buy_offer.price.clone());
    /*validations*/
    equal_accounts(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    )
    .unwrap();
    equal_accounts(ctx.accounts.buy_offer.key(), ctx.accounts.buyer_pda.key()).unwrap();
    equal_price(amount, ctx.accounts.buy_offer.price[index]).unwrap();

    /*pda lamport transfer*/
    pda_transfer(
        ctx.accounts.buy_offer.to_account_info(),
        ctx.accounts.from.to_account_info(),
        amount,
    )
    .unwrap();

    /*get &mut accounts*/
    let system = &mut ctx.accounts.decentralized_exchange_system;
    let stock_account = &mut ctx.accounts.stock_account;
    let seller_account = &mut ctx.accounts.seller_account;
    let buyer_account = &mut ctx.accounts.buyer_account;
    let buy_offer = &mut ctx.accounts.buy_offer;

    /*update state*/
    system.add_historical_exchanges();
    system.sub_total_offers();
    stock_account.sub_current_offers();
    seller_account.sub_participation(buy_offer.sell_or_buy_amount[index]);
    buyer_account.add_participation(buy_offer.sell_or_buy_amount[index]);
    buy_offer.sell_or_buy_amount.remove(index);
    buy_offer.price.remove(index);
    buy_offer.sub_len(PRODUCT);

    Ok(())
}

#[derive(Accounts)]
pub struct AcceptABuy<'info> {
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
        seeds = [b"Buy Account", stock_account_pda.key().as_ref(), from.key().as_ref()],
        bump = buy_offer.bump_original,
        realloc = 8 + buy_offer.len as usize - PRODUCT as usize,
        realloc::payer = from,
        realloc::zero = false,
    )]
    pub buy_offer: Account<'info, SellOrBuyAccount>,

    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stock_account_pda: AccountInfo<'info>,

    /// CHECK: This is not dangerous
    #[account(mut)]
    pub buyer_pda: AccountInfo<'info>,

    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
