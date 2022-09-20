use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("Ca8tecWTapYzeGfa8FvAMSo6JCheTRPvQhsjebZm56YE");

#[program]
pub mod solotery {
    use super::*;
    pub fn register_enterprise(
        ctx: Context<RegisterEnterprise>
    ) -> Result<()> {
        let enterprise: &mut Account<Owners> = &mut ctx.accounts.solotery;
        solotery.players = [].to_vec();
        Ok(())
    }
}
#[derive(Accounts)]
pub struct RegisterEnterprise<'info> {
    #[account(init, seeds = [user.key().as_ref()], bump, payer = user, space = 9612)]
    pub enterprise: Account<'info, Owners>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Owners {
    pub users: Vec<Pubkey>, // 300 owners
}
#[error_code]
pub enum ErrorCode {

}