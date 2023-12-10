use crate::{state::accounts::*, utils::utils::*};
use anchor_lang::prelude::*;

pub fn init_sell_account(ctx: Context<InitSellAccount>) -> Result<()> {
    let (_sell_pda, bump) = Pubkey::find_program_address(
        &[
            b"Sell Account",
            ctx.accounts.stock_account.key().as_ref(),
            ctx.accounts.from.key().as_ref(),
        ],
        ctx.program_id,
    );

    //validations
    require_keys_eq!(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    );
    require_keys_eq!(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    );

    //get &mut accounts
    let sell_offer = &mut ctx.accounts.sell_offer;

    //update state
    sell_offer.set_bump(bump);
    sell_offer.init_sell_or_buy_amount();
    sell_offer.init_price();
    sell_offer.set_pubkey(ctx.accounts.from.key());
    sell_offer.set_len(SELL_ACCOUNT);

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
