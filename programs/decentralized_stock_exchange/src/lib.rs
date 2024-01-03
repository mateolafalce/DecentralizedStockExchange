use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    accept_a_buy::accept_a_buy, accept_a_sell::accept_a_sell, buy_offer::buy_offer,
    cancel_buy::cancel_buy, cancel_sell::cancel_sell, create_stock::create_stock,
    init_buy_account::init_buy_account, init_holder_account::init_holder_account,
    init_sell_account::init_sell_account, initialize::initialize_decentralized_exchange_system,
    ipo_buy::buy_in_initial_public_offering, sell_offer::sell_offer,
};

declare_id!("2A65abYAkY9pBw6rAZyu5q4uwLdjyjd66WWwazewHPmv");

#[program]
pub mod decentralized_exchange {
    use super::*;

    pub fn initialize_decentralized_exchange_system_(ctx: Context<Initialize>) -> Result<()> {
        initialize_decentralized_exchange_system(ctx)
    }

    pub fn create_stock_(
        ctx: Context<CreateStock>,
        name: String,
        total_supply: u64,
        dividends: bool,
        dividend_payment_period: i64,
        date_to_go_public: i64,
        price_to_go_public: u64,
    ) -> Result<()> {
        create_stock(
            ctx,
            name,
            total_supply,
            dividends,
            dividend_payment_period,
            date_to_go_public,
            price_to_go_public,
        )
    }

    pub fn init_holder_account_(ctx: Context<InitHolderAccount>) -> Result<()> {
        init_holder_account(ctx)
    }

    pub fn buy_in_initial_public_offering_(
        ctx: Context<BuyInitialPublicOffering>,
        amount: u64,
    ) -> Result<()> {
        buy_in_initial_public_offering(ctx, amount)
    }

    pub fn init_sell_account_(ctx: Context<InitSellAccount>) -> Result<()> {
        init_sell_account(ctx)
    }

    pub fn sell_offer_(ctx: Context<SellOffer>, sell_amount: u64, price: u64) -> Result<()> {
        sell_offer(ctx, sell_amount, price)
    }

    pub fn cancel_sell_(ctx: Context<CancelSellOffer>, price_to_cancel: u64) -> Result<()> {
        cancel_sell(ctx, price_to_cancel)
    }

    pub fn init_buy_account_(ctx: Context<InitBuyAccount>) -> Result<()> {
        init_buy_account(ctx)
    }

    pub fn buy_offer_(ctx: Context<BuyOffer>, buy_amount: u64, price: u64) -> Result<()> {
        buy_offer(ctx, buy_amount, price)
    }

    pub fn cancel_buy_(ctx: Context<CancelBuyOffer>, price_to_cancel: u64) -> Result<()> {
        cancel_buy(ctx, price_to_cancel)
    }

    pub fn accept_a_sell_(ctx: Context<AcceptASell>, amount: u64) -> Result<()> {
        accept_a_sell(ctx, amount)
    }

    pub fn accept_a_buy_(ctx: Context<AcceptABuy>, amount: u64) -> Result<()> {
        accept_a_buy(ctx, amount)
    }
}
