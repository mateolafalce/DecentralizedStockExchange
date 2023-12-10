use crate::{state::accounts::*, utils::utils::*, validations::*};
use anchor_lang::prelude::*;

pub fn cancel_buy(ctx: Context<CancelBuyOffer>, price_to_cancel: u64) -> Result<()> {
    let index: usize = get_index(ctx.accounts.buy_offer.price.clone());

    //validations
    equal_accounts(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    )
    .unwrap();
    equal_accounts(ctx.accounts.buy_offer.key(), ctx.accounts.buy_pda.key()).unwrap();
    equal_price(price_to_cancel, ctx.accounts.buy_offer.price[index]).unwrap();

    //sign tx
    pda_transfer(
        ctx.accounts.buy_pda.to_account_info(),
        ctx.accounts.from.to_account_info(),
        price_to_cancel,
    )
    .unwrap();

    //get &mut accounts
    let system = &mut ctx.accounts.decentralized_exchange_system;
    let stock_account = &mut ctx.accounts.stock_account;
    let buy_offer = &mut ctx.accounts.buy_offer;

    //update state
    buy_offer.sell_or_buy_amount.remove(index);
    buy_offer.price.remove(index);
    buy_offer.sub_len(PRODUCT);
    system.sub_total_offers();
    stock_account.sub_current_offers();

    Ok(())
}

#[derive(Accounts)]
pub struct CancelBuyOffer<'info> {
    #[account(mut, seeds = [b"System Account"], bump = decentralized_exchange_system.bump_original)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,

    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,

    #[account(
        mut,
        seeds = [b"Buy Account", stock_account.key().as_ref(), from.key().as_ref()],
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
    pub buy_pda: AccountInfo<'info>,

    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
