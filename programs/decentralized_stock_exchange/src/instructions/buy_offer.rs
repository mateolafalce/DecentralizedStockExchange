use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn buy_offer(
        ctx: Context<BuyOffer>,
        buy_amount: u64,
        price: u64
    ) -> Result<()> {
        let (holder_pda, _bump) = Pubkey::find_program_address(&[ctx.accounts.stock_account.key().as_ref(), ctx.accounts.from.key().as_ref()], ctx.program_id);
        require!(holder_pda.key() == ctx.accounts.holder_account.key(), ErrorCode::HolderError);
        require!(buy_amount > 0, ErrorCode::AmountError);
        require!(ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(), ErrorCode::PubkeyError);
        require!(buy_amount <= ctx.accounts.stock_account.total_supply, ErrorCode::AmountError);
        require!(ctx.accounts.buy_offer.key() == ctx.accounts.buy_pda.key(), ErrorCode::PubkeyError);
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.buy_offer.key(), price),
            &[ctx.accounts.from.to_account_info(), ctx.accounts.buy_pda.to_account_info().clone()],
        ).expect("Error");
        let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system; 
        let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
        let holder_account: &mut Account<HolderAccount> = &mut ctx.accounts.holder_account;
        let buy_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.buy_offer; 
        buy_offer.sell_or_buy_amount.push(buy_amount);
        buy_offer.price.push(price);
        buy_offer.len += 16;
        system.total_offers += 1;
        stock_account.current_offers += 1;
        Ok(())
    }

#[derive(Accounts)]
pub struct BuyOffer<'info> {
    #[account(mut, seeds = [b"System Account"], bump = decentralized_exchange_system.bump_original)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,
    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,
    #[account(
        mut,
        seeds = [b"Buy Account", stock_account_pda.key().as_ref(), from.key().as_ref()],
        bump = buy_offer.bump_original,
        realloc = 8 + buy_offer.len as usize + 16,
        realloc::payer = from,
        realloc::zero = false,
    )]
    pub buy_offer: Account<'info, SellOrBuyAccount>,
    #[account(mut, seeds = [stock_account_pda.key().as_ref(), from.key().as_ref()], bump = holder_account.bump_original)]
    pub holder_account: Account<'info, HolderAccount>,
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