use crate::{state::accounts::*, utils::util::*};
use anchor_lang::prelude::*;

pub fn sell_offer(ctx: Context<SellOffer>, sell_amount: u64, price: u64) -> Result<()> {
    let (holder_pda, _bump) = Pubkey::find_program_address(
        &[
            ctx.accounts.stock_account.key().as_ref(),
            ctx.accounts.from.key().as_ref(),
        ],
        ctx.program_id,
    );
    ctx.accounts.sell_offer.price.push(price);

    //validations
    require_keys_eq!(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    );
    require_keys_eq!(holder_pda.key(), ctx.accounts.holder_account.key());
    require_gte!(ctx.accounts.holder_account.participation, sell_amount);
    require_eq!(unique_elements(vec![&ctx.accounts.sell_offer.price]), true);
    require_gt!(sell_amount, 0);

    //get &mut accounts
    let system = &mut ctx.accounts.decentralized_exchange_system;
    let stock_account = &mut ctx.accounts.stock_account;
    let sell_offer = &mut ctx.accounts.sell_offer;
    let holder_account = &mut ctx.accounts.holder_account;

    //update state
    sell_offer.sell_or_buy_amount.push(sell_amount);
    sell_offer.price.push(price);
    sell_offer.add_len(PRODUCT);
    system.add_total_offers();
    stock_account.add_current_offers();
    holder_account.sub_participation(sell_amount);

    Ok(())
}

#[derive(Accounts)]
pub struct SellOffer<'info> {
    #[account(mut, seeds = [b"System Account"], bump = decentralized_exchange_system.bump_original)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,

    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,

    #[account(mut, seeds = [stock_account_pda.key().as_ref(), from.key().as_ref()], bump = holder_account.bump_original)]
    pub holder_account: Account<'info, HolderAccount>,

    #[account(
        mut,
        seeds = [b"Sell Account", stock_account_pda.key().as_ref(), from.key().as_ref()],
        bump = sell_offer.bump_original,
        realloc = 8 + sell_offer.len as usize + 16,
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
    pub system_program: Program<'info, System>,
}
