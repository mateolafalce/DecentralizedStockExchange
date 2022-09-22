use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("DfvjgjoyxkJfydqhxwjzkjvmaxjEmzpJYpVdoLnsqvgH");

#[program]
pub mod decentralized_stock_exchange {
    use super::*;
    pub fn register_enterprise(
        ctx: Context<RegisterEnterprise>,
        market_release_amount: u64,
        release_day: i64
    ) -> Result<()> {
        let system_program: Pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let enterprise: &mut Account<Owners> = &mut ctx.accounts.enterprise;
        let offers: &mut Account<Offers> = &mut ctx.accounts.offers;
        enterprise.authority = ctx.accounts.user.key();
        offers.authority = ctx.accounts.user.key();
        enterprise.owners = [system_program; 300].to_vec();
        offers.offers_amount = [market_release_amount; 300].to_vec();
        offers.offer_state = [true; 300].to_vec();
        enterprise.release_day = release_day;
        Ok(())
    }
    pub fn buy(
        ctx: Context<Buy>,
        _number: u16,
        amount: u64
    ) -> Result<()> {
        let number: usize = _number as usize;
        let (enterprise_pda, _bump) = Pubkey::find_program_address(&[ctx.accounts.from.key().as_ref()], &Pubkey::from_str("DfvjgjoyxkJfydqhxwjzkjvmaxjEmzpJYpVdoLnsqvgH").unwrap());
        let (offers_pda, _bump) = Pubkey::find_program_address(&[b"Offers", ctx.accounts.from.key().as_ref()], &Pubkey::from_str("DfvjgjoyxkJfydqhxwjzkjvmaxjEmzpJYpVdoLnsqvgH").unwrap());
        require!(ctx.accounts.enterprise.key() == enterprise_pda, ErrorCode::IncorrectOwner);
        require!(ctx.accounts.offers.key() == offers_pda, ErrorCode::IncorrectOwner);
        let system_program: Pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        require!(number < 300, ErrorCode::IncorrectNumber);
        if ctx.accounts.enterprise.owners[number] == system_program {
            require!(ctx.accounts.to.key() == ctx.accounts.enterprise.receiver, ErrorCode::IncorrectReceiver);
            require!(AccountInfo::lamports(&ctx.accounts.from.to_account_info()) > ctx.accounts.offers.offers_amount[number], ErrorCode::InsufficientSOL);
            anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.enterprise.receiver, ctx.accounts.offers.offers_amount[number]),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.to.to_account_info().clone()],
            ).expect("Error");
            let enterprise: &mut Account<Owners> = &mut ctx.accounts.enterprise;
            let offers: &mut Account<Offers> = &mut ctx.accounts.offers;
            enterprise.owners[number] = ctx.accounts.from.key();
            offers.offer_state[number] = false;
        }
        if ctx.accounts.enterprise.owners[number] != system_program {
            require!(ctx.accounts.to.key() == ctx.accounts.enterprise.owners[number], ErrorCode::IncorrectReceiver);
            require!(ctx.accounts.offers.offer_state[number] == true, ErrorCode::ThereIsNoOffer);
            require!(AccountInfo::lamports(&ctx.accounts.from.to_account_info()) > ctx.accounts.offers.offers_amount[number], ErrorCode::InsufficientSOL);
            anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.enterprise.owners[number], ctx.accounts.offers.offers_amount[number]),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.to.to_account_info().clone()],
            ).expect("Error");
            let enterprise: &mut Account<Owners> = &mut ctx.accounts.enterprise;
            let offers: &mut Account<Offers> = &mut ctx.accounts.offers;
            enterprise.owners[number] = ctx.accounts.from.key();
            offers.offers_amount[number] = amount;
            offers.offer_state[number] = false;
        }
        Ok(())
    }
    pub fn sell_change_too(
        ctx: Context<Sell>,
        _number: u16,
        amount: u64
    ) -> Result<()> {
        let number: usize = _number as usize;
        let (enterprise_pda, _bump) = Pubkey::find_program_address(&[ctx.accounts.from.key().as_ref()], &Pubkey::from_str("DfvjgjoyxkJfydqhxwjzkjvmaxjEmzpJYpVdoLnsqvgH").unwrap());
        let (offers_pda, _bump) = Pubkey::find_program_address(&[b"Offers", ctx.accounts.from.key().as_ref()], &Pubkey::from_str("DfvjgjoyxkJfydqhxwjzkjvmaxjEmzpJYpVdoLnsqvgH").unwrap());
        require!(ctx.accounts.enterprise.key() == enterprise_pda, ErrorCode::IncorrectOwner);
        require!(ctx.accounts.offers.key() == offers_pda, ErrorCode::IncorrectOwner);
        require!(ctx.accounts.enterprise.owners[number] == ctx.accounts.from.key(), ErrorCode::IncorrectOwner);
        let offers: &mut Account<Offers> = &mut ctx.accounts.offers;
        offers.offers_amount[number] = amount;
        offers.offer_state[number] = true;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct RegisterEnterprise<'info> {
    #[account(init, seeds = [user.key().as_ref()], bump, payer = user, space = 9685)]
    pub enterprise: Account<'info, Owners>,
    #[account(init, seeds = [b"Offers", user.key().as_ref()], bump, payer = user, space = 2749)]
    pub offers: Account<'info, Offers>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut, seeds = [enterprise.authority.key().as_ref()], bump = enterprise.bump_original)]
    pub enterprise: Account<'info, Owners>,
    #[account(mut, seeds = [b"Offers", offers.authority.key().as_ref()], bump = offers.bump_original)]
    pub offers: Account<'info, Offers>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut, seeds = [enterprise.authority.key().as_ref()], bump = enterprise.bump_original)]
    pub enterprise: Account<'info, Owners>,
    #[account(mut, seeds = [b"Offers", offers.authority.key().as_ref()], bump = offers.bump_original)]
    pub offers: Account<'info, Offers>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>
}
#[account]
pub struct Owners {
    pub authority: Pubkey, 
    pub receiver: Pubkey,
    pub bump_original: u8, 
    pub owners: Vec<Pubkey>, // 300 owners
    pub release_day: i64,
}
#[account]
pub struct Offers {
    pub authority: Pubkey, 
    pub bump_original: u8, 
    pub offers_amount: Vec<u64>, // 300 offers
    pub offer_state: Vec<bool>, // 300 states
}
#[error_code]
pub enum ErrorCode {
    #[msg("Only 0 to 299")]IncorrectNumber, #[msg("This is not the initial offer")]NotTheInitialOffer, 
    #[msg("This is not the recipient")]IncorrectReceiver, #[msg("The owner has not made a proposal")]ThereIsNoOffer, 
    #[msg("You don't have the necessary lamports for this action")]InsufficientSOL, 
    #[msg("You are not the owner")]IncorrectOwner, 
}