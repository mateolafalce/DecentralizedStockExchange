use crate::errors::ErrorCode;
use crate::state::accounts::*;
use anchor_lang::{
    prelude::*, solana_program::account_info::AccountInfo, solana_program::pubkey::Pubkey,
};

pub fn init_sell_account(ctx: Context<InitSellAccount>) -> Result<()> {
    let (_sell_pda, bump) = Pubkey::find_program_address(
        &[
            b"Sell Account",
            ctx.accounts.stock_account.key().as_ref(),
            ctx.accounts.from.key().as_ref(),
        ],
        ctx.program_id,
    );
    // Ensure that the stock account PDA is the same as the stock account
    require!(
        ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(),
        ErrorCode::PubkeyError
    );
    // Ensure that the stock account PDA is the same as the stock account
    require!(
        ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(),
        ErrorCode::PubkeyError
    );
    let sell_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.sell_offer;
    // Set the original bump value for the sell offer account
    sell_offer.bump_original = bump;
    // Initialize sell_or_buy_amount and price fields as empty vectors
    sell_offer.sell_or_buy_amount = [].to_vec();
    sell_offer.price = [].to_vec();
    // Set the pubkey field of the sell offer account as the "from" account's key
    sell_offer.pubkey = ctx.accounts.from.key();
    sell_offer.len = 65;
    Ok(())
}

#[derive(Accounts)]
pub struct InitSellAccount<'info> {
    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,
    #[account(init, seeds = [b"Sell Account", stock_account_pda.key().as_ref(), from.key().as_ref()], bump, payer = from, space = SellOrBuyAccount::SIZE + 8)]
    pub sell_offer: Account<'info, SellOrBuyAccount>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stock_account_pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
