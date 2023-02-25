use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn accept_a_sell(
        ctx: Context<AcceptASell>,
        price: u64
    ) -> Result<()> {
        require!(ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(), ErrorCode::PubkeyError);
        require!(ctx.accounts.sell_offer.key() == ctx.accounts.sell_pda.key(), ErrorCode::PubkeyError);
        let index = ctx.accounts.sell_offer.price.iter().position(|&price| price == price).unwrap();
        require!(price == ctx.accounts.sell_offer.price[index], ErrorCode::PriceError);
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.sell_offer.key(), ctx.accounts.sell_offer.price[index]),
            &[ctx.accounts.from.to_account_info(), ctx.accounts.sell_pda.to_account_info().clone()],
        ).expect("Error");
        let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system; 
        let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
        let seller_account: &mut Account<HolderAccount> = &mut ctx.accounts.seller_account;
        let buyer_account: &mut Account<HolderAccount> = &mut ctx.accounts.buyer_account;
        let sell_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.sell_offer; 
        system.historical_exchanges += 1;
        system.total_offers -= 1;
        stock_account.current_offers -= 1;
        seller_account.participation -= sell_offer.sell_or_buy_amount[index];
        buyer_account.participation += sell_offer.sell_or_buy_amount[index];
        sell_offer.sell_or_buy_amount.remove(index);
        sell_offer.price.remove(index);
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
    pub system_program: Program<'info, System>
}