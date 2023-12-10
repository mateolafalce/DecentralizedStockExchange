use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("2A65abYAkY9pBw6rAZyu5q4uwLdjyjd66WWwazewHPmv");

#[program]
pub mod decentralized_exchange {
    use super::*;

    pub fn initialize_decentralized_exchange_system(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize_decentralized_exchange_system(ctx)
    }

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
        instructions::create_stock::create_stock(
            ctx,
            name,
            description,
            total_supply,
            dividends,
            dividend_payment_period,
            date_to_go_public,
            price_to_go_public,
        )
    }

    pub fn init_holder_account(ctx: Context<InitHolderAccount>) -> Result<()> {
        instructions::init_holder_account::init_holder_account(ctx)
    }

    pub fn buy_in_initial_public_offering(
        ctx: Context<BuyInitialPublicOffering>,
        amount: u64,
    ) -> Result<()> {
        instructions::ipo_buy::buy_in_initial_public_offering(ctx, amount)
    }

    pub fn init_sell_account(ctx: Context<InitSellAccount>) -> Result<()> {
        instructions::init_sell_account::init_sell_account(ctx)
    }

    pub fn sell_offer(ctx: Context<SellOffer>, sell_amount: u64, price: u64) -> Result<()> {
        instructions::sell_offer::sell_offer(ctx, sell_amount, price)
    }

    pub fn cancel_sell(ctx: Context<CancelSellOffer>, price_to_cancel: u64) -> Result<()> {
        instructions::cancel_sell::cancel_sell(ctx, price_to_cancel)
    }

    pub fn init_buy_account(ctx: Context<InitBuyAccount>) -> Result<()> {
        instructions::init_buy_account::init_buy_account(ctx)
    }

    pub fn buy_offer(ctx: Context<BuyOffer>, buy_amount: u64, price: u64) -> Result<()> {
        instructions::buy_offer::buy_offer(ctx, buy_amount, price)
    }

    pub fn cancel_buy(ctx: Context<CancelBuyOffer>, price_to_cancel: u64) -> Result<()> {
        instructions::cancel_buy::cancel_buy(ctx, price_to_cancel)
    }

    pub fn accept_a_sell(ctx: Context<AcceptASell>, amount: u64) -> Result<()> {
        instructions::accept_a_sell::accept_a_sell(ctx, amount)
    }

    pub fn accept_a_buy(ctx: Context<AcceptABuy>, amount: u64) -> Result<()> {
        instructions::accept_a_buy::accept_a_buy(ctx, amount)
    }
}
