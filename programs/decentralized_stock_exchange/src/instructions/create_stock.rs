use crate::{state::accounts::*, utils::utils::*, validations::*};
use anchor_lang::prelude::*;

pub fn create_stock(
    ctx: Context<CreateStock>,
    name: String,
    description: String,
    total_supply: u64,
    dividends: bool,
    dividend_payment_period: i64,
    date_to_go_public: i64,
    price_to_go_public: u64,
) -> Result<()> {
    let (_stock_pda, bump) = Pubkey::find_program_address(
        &[b"Stock Account", ctx.accounts.from.key().as_ref()],
        ctx.program_id,
    );

    //validations
    less_or_equal_than(name.len() as u64, NAME).unwrap();
    less_or_equal_than(description.len() as u64, DESCRIPTION).unwrap();
    check_current_time(date_to_go_public).unwrap();

    //get &mut accounts
    let system = &mut ctx.accounts.decentralized_exchange_system;
    let stock_account = &mut ctx.accounts.stock_account;

    //update state
    system.add_total_stock_companies();
    stock_account.set_bump(bump);
    stock_account.set_pubkey(ctx.accounts.from.key());
    stock_account.set_name(name);
    stock_account.set_description(description);
    stock_account.set_total_supply(total_supply);
    stock_account.set_supply_in_position(total_supply);
    stock_account.init_holders();
    stock_account.set_dividends(dividends);
    stock_account.set_dividend_payment_period(dividend_payment_period);
    stock_account.set_date_to_go_public(date_to_go_public);
    stock_account.set_price_to_go_public(price_to_go_public);

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
