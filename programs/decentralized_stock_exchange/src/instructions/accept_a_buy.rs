use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn accept_a_buy(
        ctx: Context<AcceptABuy>,
        price: u64
    ) -> Result<()> {
        require!(ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(), ErrorCode::PubkeyError);
        require!(ctx.accounts.buy_offer.key() == ctx.accounts.buyer_pda.key(), ErrorCode::PubkeyError);
        let index = ctx.accounts.buy_offer.price.iter().position(|&price| price == price).unwrap();
        require!(price == ctx.accounts.buy_offer.price[index], ErrorCode::PriceError);
        **ctx.accounts.buy_offer.to_account_info().try_borrow_mut_lamports()? -= price;
        **ctx.accounts.from.to_account_info().try_borrow_mut_lamports()? += price;
        let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system; 
        let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
        let seller_account: &mut Account<HolderAccount> = &mut ctx.accounts.seller_account;
        let buyer_account: &mut Account<HolderAccount> = &mut ctx.accounts.buyer_account;
        let buy_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.buy_offer; 
        system.historical_exchanges += 1;
        system.total_offers -= 1;
        stock_account.current_offers -= 1;
        seller_account.participation -= buy_offer.sell_or_buy_amount[index];
        buyer_account.participation += buy_offer.sell_or_buy_amount[index];//I
        buy_offer.sell_or_buy_amount.remove(index);
        buy_offer.price.remove(index);
        buy_offer.len -= 16;
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
        realloc = 8 + buy_offer.len as usize - 16,
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
    pub system_program: Program<'info, System>
}