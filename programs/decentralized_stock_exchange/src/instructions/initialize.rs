use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;

pub fn initialize_decentralized_exchange_system(
    ctx: Context<Initialize>
) -> Result<()> {
    // Get a mutable reference to the decentralized exchange system account
    let system: &mut Account<SystemExchangeAccount> = &mut ctx.accounts.decentralized_exchange_system;
    // Find the program address using the seed "System Account" and program ID
    let (_pda, bump) = Pubkey::find_program_address(&[b"System Account"], ctx.program_id);
    // Set the bump value for the system account
    system.bump_original = bump;
    // Initialize various system account fields
    system.total_stock_companies = 0;
    system.historical_exchanges = 0;
    system.total_holders = 0;
    system.total_offers = 0;
    Ok(()) 
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"System Account"], bump, payer = user, space = SystemExchangeAccount::SIZE + 8)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>, // System account to be initialized
    #[account(mut)]
    pub user: Signer<'info>, // User account (mutable)
    pub system_program: Program<'info, System>, // System program account
}
