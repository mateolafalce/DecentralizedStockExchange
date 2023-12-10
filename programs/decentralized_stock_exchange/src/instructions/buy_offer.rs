use crate::{state::accounts::*, utils::utils::PRODUCT, validations::*};
use anchor_lang::{prelude::*, solana_program::*};

pub fn buy_offer(ctx: Context<BuyOffer>, buy_amount: u64, price: u64) -> Result<()> {
    let (holder_pda, _bump) = Pubkey::find_program_address(
        &[
            ctx.accounts.stock_account.key().as_ref(),
            ctx.accounts.from.key().as_ref(),
        ],
        ctx.program_id,
    );

    //validations
    equal_accounts(holder_pda.key(), ctx.accounts.holder_account.key()).unwrap();
    equal_accounts(ctx.accounts.buy_offer.key(), ctx.accounts.buy_pda.key()).unwrap();
    equal_accounts(
        ctx.accounts.stock_account_pda.key(),
        ctx.accounts.stock_account.key(),
    )
    .unwrap();
    less_or_equal_than(buy_amount, ctx.accounts.stock_account.total_supply).unwrap();
    greater_than_0(buy_amount).unwrap();

    // lamports transfer
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.buy_offer.key(),
            price,
        ),
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.buy_pda.to_account_info().clone(),
        ],
    )
    .expect("Error");

    //get &mut accounts
    let system = &mut ctx.accounts.decentralized_exchange_system;
    let stock_account = &mut ctx.accounts.stock_account;
    let buy_offer = &mut ctx.accounts.buy_offer;

    //update state
    buy_offer.sell_or_buy_amount.push(buy_amount);
    buy_offer.price.push(price);
    buy_offer.add_len(PRODUCT);
    system.add_total_offers();
    stock_account.add_current_offers();

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
        realloc = 8 + buy_offer.len as usize + PRODUCT as usize,
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
    pub system_program: Program<'info, System>,
}
