use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn init_buy_account(
        ctx: Context<InitBuyAccount>
    ) -> Result<()> {
        let (_buy_pda, bump) = Pubkey::find_program_address(&[b"Buy Account", ctx.accounts.stock_account_pda.key().as_ref(), ctx.accounts.from.key().as_ref()], ctx.program_id);
        require!(ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(), ErrorCode::PubkeyError);
        let buy_offer: &mut Account<SellOrBuyAccount> = &mut ctx.accounts.buy_offer; 
        buy_offer.bump_original = bump;
        buy_offer.sell_or_buy_amount = [].to_vec();
        buy_offer.price = [].to_vec();
        buy_offer.pubkey = ctx.accounts.from.key();
        buy_offer.len = 65;
        Ok(())
    }

#[derive(Accounts)]
pub struct InitBuyAccount<'info> {
    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,
    #[account(init, seeds = [b"Buy Account", stock_account.key().as_ref(), from.key().as_ref()], bump, payer = from, space = SellOrBuyAccount::SIZE + 8)]
    pub buy_offer: Account<'info, SellOrBuyAccount>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stock_account_pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}