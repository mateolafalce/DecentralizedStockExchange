use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn cancel_buy(
        ctx: Context<CancelBuyOffer>,
        price_to_cancel: u64
    ) -> Result<()> {
        require!(ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(), ErrorCode::PubkeyError);
        require!(ctx.accounts.buy_offer.key() == ctx.accounts.buy_pda.key(), ErrorCode::PubkeyError);
        let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system; 
        let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
        let buy_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.buy_offer; 
        let index = buy_offer.price.iter().position(|&price| price == price_to_cancel).unwrap();
        require!(price_to_cancel == buy_offer.price[index], ErrorCode::PriceError);
        buy_offer.sell_or_buy_amount.remove(index);
        buy_offer.price.remove(index);
        buy_offer.len -= 16;
        system.total_offers -= 1;
        stock_account.current_offers -= 1;
        **ctx.accounts.buy_pda.to_account_info().try_borrow_mut_lamports()? -= price_to_cancel;
        **ctx.accounts.from.to_account_info().try_borrow_mut_lamports()? += price_to_cancel;
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
    pub buy_pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}