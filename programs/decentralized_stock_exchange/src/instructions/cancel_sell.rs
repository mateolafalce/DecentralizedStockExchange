use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn cancel_sell(
        ctx: Context<CancelSellOffer>,
        price_to_cancel: u64
    ) -> Result<()> {
        let (holder_pda, _bump) = Pubkey::find_program_address(&[ctx.accounts.stock_account.key().as_ref(), ctx.accounts.from.key().as_ref()], ctx.program_id);
        require!(price_to_cancel > 0, ErrorCode::AmountError);
        require!(ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(), ErrorCode::PubkeyError);
        require!(holder_pda.key() == ctx.accounts.holder_account.key(), ErrorCode::HolderError);
        //require!(price_to_cancel <= ctx.accounts.holder_account.commercial_participation, ErrorCode::AmountError);
        let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system; 
        let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
        let holder_account: &mut Account<HolderAccount> = &mut ctx.accounts.holder_account;
        let sell_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.sell_offer;
        let index = sell_offer.price.iter().position(|&price| price == price_to_cancel).unwrap();
        require!(price_to_cancel == sell_offer.price[index], ErrorCode::PriceError);
        let index = sell_offer.price.iter().position(|&price| price == price_to_cancel).unwrap(); 
        sell_offer.sell_or_buy_amount.remove(index);
        sell_offer.price.remove(index);
        sell_offer.len -= 16;
        system.total_offers -= 1;
        stock_account.current_offers -= 1;
        holder_account.participation += price_to_cancel;
        Ok(())
    }

#[derive(Accounts)]
pub struct CancelSellOffer<'info> {
    #[account(mut, seeds = [b"System Account"], bump = decentralized_exchange_system.bump_original)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,
    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,
    #[account(mut, seeds = [stock_account_pda.key().as_ref(), from.key().as_ref()], bump = holder_account.bump_original)]
    pub holder_account: Account<'info, HolderAccount>,
    #[account(
        mut,
        seeds = [b"Sell Account", stock_account.key().as_ref(), from.key().as_ref()],
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
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}