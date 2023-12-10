use crate::state::accounts::*;
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn initialize_decentralized_exchange_system(ctx: Context<Initialize>) -> Result<()> {
    let system = &mut ctx.accounts.decentralized_exchange_system;
    let (_pda, bump) = Pubkey::find_program_address(&[b"System Account"], ctx.program_id);

    //update state
    system.set_bump(bump);
    system.init_stock_companies();
    system.init_historical_exchanges();
    system.init_total_holders();
    system.init_total_offers();

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"System Account"], bump, payer = user, space = SystemExchangeAccount::SIZE + 8)]
    pub decentralized_exchange_system: Account<'info, SystemExchangeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
