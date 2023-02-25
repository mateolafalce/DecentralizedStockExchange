use anchor_lang::prelude::*;

#[account]
pub struct SystemExchangeAccount { 
    pub bump_original: u8,               // 1
    pub total_stock_companies: u32,      // 4
    pub historical_exchanges: u64,       // 8
    pub total_holders: u64,              // 8
    pub total_offers: u64                // 8
}

#[account]
pub struct StockAccount {
    pub bump_original: u8,              // 1
    pub pubkey_original: Pubkey,        // 32
    pub name: String,                   // 4 + 50
    pub description: String,            // 4 + 200
    pub total_supply: u64,              // 8
    pub supply_in_position: u64,        // 8
    pub holders: u64,                   // 8
    pub dividends: bool,                // 1
    pub dividend_payment_period: i64,   // 8
    pub date_to_go_public: i64,         // 8 
    pub price_to_go_public: u64,        // 8
    pub current_offers: u32,            // 4
}

#[account]
pub struct HolderAccount { 
    pub bump_original: u8,               // 1
    pub participation: u64,              // 8
    pub holder_pubkey: Pubkey            // 32
}

#[account]
pub struct SellOrBuyAccount { 
    pub bump_original: u8,               // 1
    pub sell_or_buy_amount: Vec<u64>,    // 4 + 8
    pub price: Vec<u64>,                 // 4 + 8
    pub pubkey: Pubkey,                  // 32
    pub len: u64                         // 8
}

impl SellOrBuyAccount {
    pub const SIZE: usize = 1 + 4 + 8 + 4 + 8 + 32 + 8;
}

impl HolderAccount {
    pub const SIZE: usize = 1 + 8 + 32;
}

impl StockAccount {
    pub const SIZE: usize =  1 + 32 + 4 + 50 + 4 + 200 + 8 + 8 + 1 + 8 + 8 + 8 + 4 + 8;
}

impl SystemExchangeAccount {
    pub const SIZE: usize =  1 + 4 + 8 + 8 + 8;
}
