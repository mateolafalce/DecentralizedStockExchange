use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn buy_in_initial_public_offering(
    ctx: Context<BuyInitialPublicOffering>,
    amount: u64
) -> Result<()> {
    let (holder_pda, _bump) = Pubkey::find_program_address(
        &[ctx.accounts.stock_account.key().as_ref(), ctx.accounts.from.key().as_ref()],
        ctx.program_id,
    );
    // Check if the amount is greater than zero
    require!(amount > 0, ErrorCode::AmountError);
    // Check if the stock account PDA is the same as the stock account
    require!(
        ctx.accounts.stock_account_pda.key() == ctx.accounts.stock_account.key(),
        ErrorCode::PubkeyError
    );
    // Check if the amount is less than or equal to the total supply of the stock account
    require!(
        amount <= ctx.accounts.stock_account.total_supply,
        ErrorCode::SupplyError
    );
    // Check if the holder PDA is the same as the holder account
    require!(holder_pda.key() == ctx.accounts.holder_account.key(), ErrorCode::HolderError);
    // Calculate the amount to send based on the price to go public and the amount of stock to buy
    let amount_to_send: u64 = ctx.accounts.stock_account.price_to_go_public * amount;
    // Transfer funds from the "from" account to the stock account PDA
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.stock_account_pda.key(),
            amount_to_send,
        ),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.stock_account_pda.to_account_info().clone()],
    ).expect("Error");
    // Get mutable references to the system, holder, and stock accounts
    let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system;
    let holder_account: &mut Account<HolderAccount> = &mut ctx.accounts.holder_account;
    let stock_account: &mut Account<StockAccount> = &mut ctx.accounts.stock_account;
    // Update the holder account participation
    holder_account.participation = amount;
    // Decrease the supply in position of the stock account
    stock_account.supply_in_position -= amount;
    // Increase the number of historical exchanges in the system
    system.historical_exchanges += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct BuyInitialPublicOffering<'info> {
    #[account(mut, seeds = [b"System Account"], bump = decentralized_exchange_system.bump_original)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,
    #[account(mut, seeds = [b"Stock Account", stock_account.pubkey_original.key().as_ref()], bump = stock_account.bump_original)]
    pub stock_account: Account<'info, StockAccount>,
    #[account(mut, seeds = [stock_account_pda.key().as_ref(), from.key().as_ref()], bump = holder_account.bump_original)]
    pub holder_account: Account<'info, HolderAccount>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stock_account_pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}
