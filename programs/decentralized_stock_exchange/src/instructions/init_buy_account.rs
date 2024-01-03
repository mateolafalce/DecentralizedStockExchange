use crate::{state::accounts::*, utils::util::*};
use anchor_lang::prelude::*;

pub fn init_buy_account(ctx: Context<InitBuyAccount>) -> Result<()> {
    let (_buy_pda, bump) = Pubkey::find_program_address(
        &[
            b"Buy Account",
            ctx.accounts.stock_account_pda.key().as_ref(),
            ctx.accounts.from.key().as_ref(),
        ],
        ctx.program_id,
    );

    //validation
    require_keys_eq!(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    );

    //get &mut account
    let buy_offer = &mut ctx.accounts.buy_offer;

    //update state
    buy_offer.set_bump(bump);
    buy_offer.init_sell_or_buy_amount();
    buy_offer.init_price();
    buy_offer.set_pubkey(ctx.accounts.from.key());
    buy_offer.set_len(BUY_ACCOUNT);

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
    pub system_program: Program<'info, System>,
}
