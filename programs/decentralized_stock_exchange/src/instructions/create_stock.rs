use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn create_stock(
    ctx: Context<CreateStock>,
    name: String,
    description: String,
    total_supply: u64,
    dividends: bool,
    dividend_payment_period: i64,
    date_to_go_public: i64,
    price_to_go_public: u64
) -> Result<()> {
    let (_stock_pda, bump) = Pubkey::find_program_address(&[b"Stock Account", ctx.accounts.from.key().as_ref()], ctx.program_id);
    // Check if the name length is within the allowed limit
    require!(name.len() <= 50, ErrorCode::NameError);
    // Check if the description length is within the allowed limit
    require!(description.len() <= 200, ErrorCode::DescriptionError);
    // Check if the specified date to go public is in the future
    require!(date_to_go_public > Clock::get().unwrap().unix_timestamp, ErrorCode::Date);
    let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system;
    let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
    // Increment the total stock companies count in the decentralized exchange system
    system.total_stock_companies += 1;
    stock_account.bump_original = bump;
    // Set the original pubkey for the stock account
    stock_account.pubkey_original = ctx.accounts.from.key();
    // Set variables
    stock_account.name = name;
    stock_account.description = description;
    stock_account.total_supply = total_supply;
    stock_account.supply_in_position = total_supply;
    stock_account.holders = 1;
    stock_account.dividends = dividends;
    // Set the dividend payment period for the stock account
    stock_account.dividend_payment_period = dividend_payment_period;
    // Set the date to go public for the stock account
    stock_account.date_to_go_public = date_to_go_public;
    // Set the price to go public for the stock account
    stock_account.price_to_go_public = price_to_go_public;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateStock<'info> {
    #[account(mut, seeds = [b"System Account"], bump = decentralized_exchange_system.bump_original)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,
    #[account(init, seeds = [b"Stock Account", from.key().as_ref()], bump, payer = from, space = StockAccount::SIZE + 8)]
    pub stock_account: Account<'info, StockAccount>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
