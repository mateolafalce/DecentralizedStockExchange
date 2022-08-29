use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod decentralized_stock_exchange {
    use super::*;

    pub fn create_stock(
        ctx: Context<CreateStockMaker>,
    ) -> Result<()> {
        if ctx.accounts.user.key() != Pubkey::from_str("9RDz3M796x25qXfVGUSau3rze3WH4z8KesZe3MMBYfrZ").unwrap() {
            return Err(ErrorCode::YouAreNotStockMaker.into());
        }
        let decentralized_stock_exchange: &mut Account<DecentralizedStockExchange> = &mut ctx.accounts.decentralized_stock_exchange;
        let (stock_maker_pda_account, bump) = Pubkey::find_program_address(&[b"DecentralizedStockExchange", ctx.accounts.user.key().as_ref()], &Pubkey::from_str("9RDz3M796x25qXfVGUSau3rze3WH4z8KesZe3MMBYfrZ").unwrap());
        decentralized_stock_exchange.authority = ctx.accounts.user.key(); 
        decentralized_stock_exchange.bump_original = bump;
        decentralized_stock_exchange.stock_maker_pda_account = stock_maker_pda_account;
        decentralized_stock_exchange.row = 1;
        decentralized_stock_exchange.column = 1;
        decentralized_stock_exchange.nivel = 1;
        decentralized_stock_exchange.total_registered_companies = 0;
        Ok(())
    }
    pub fn register(
        ctx: Context<RegisterStruct>,
        number_of_shares_to_be_issued: u32,
    ) -> Result<()> {
        if number_of_shares_to_be_issued < 1000 {
            return Err(ErrorCode::Base10000Stocks.into());
        }
        let data_holder: &mut Account<DataHolder> = &mut ctx.accounts.data_holder;
        data_holder.number_of_shares_to_be_issued = number_of_shares_to_be_issued;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateStockMaker<'info> {
    #[account(init, seeds = [b"DecentralizedStockExchange", user.key().as_ref()], bump, payer = user, space = 8 + 32 + 32 + 1 + 1 + 4 + 1)]
    pub decentralized_stock_exchange: Account<'info, DecentralizedStockExchange>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RegisterStruct<'info> {
    #[account(init, payer = from, space = 12)]
    pub data_holder: Account<'info, DataHolder>,
    #[account(mut, seeds = [b"DecentralizedStockExchange", decentralized_stock_exchange.authority.key().as_ref()], bump = decentralized_stock_exchange.bump_original)]
    pub decentralized_stock_exchange: Account<'info, DecentralizedStockExchange>,
    #[account(init, seeds = [decentralized_stock_exchange.row.to_le_bytes().as_ref(), decentralized_stock_exchange.column.to_le_bytes().as_ref()], bump, payer = from, space = 42)]
    pub enable_stock_actions: Account<'info, Stock>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct DecentralizedStockExchange {
    pub authority: Pubkey,
    pub stock_maker_pda_account: Pubkey,
    pub bump_original: u8,
    pub row: u8,
    pub column: u8,
    pub nivel: u8,
    pub total_registered_companies: u32,

}
#[account]
pub struct Stock {
    pub authority: Pubkey,
    pub row: u8,
    pub column: u8,
}
#[account]
pub struct DataHolder {
    pub number_of_shares_to_be_issued: u32
}
#[error_code]
pub enum ErrorCode {
    #[msg("You are not Stock Maker")]YouAreNotStockMaker, #[msg("Base is 10,000 stocks")]Base10000Stocks,
}