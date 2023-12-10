use anchor_lang::prelude::*;

#[account]
pub struct SystemExchangeAccount {
    pub bump_original: u8,          // 1
    pub total_stock_companies: u32, // 4
    pub historical_exchanges: u64,  // 8
    pub total_holders: u64,         // 8
    pub total_offers: u64,          // 8
} // size = 29

#[account]
pub struct StockAccount {
    pub bump_original: u8,            // 1
    pub pubkey_original: Pubkey,      // 32
    pub name: String,                 // 4 + 50
    pub description: String,          // 4 + 200
    pub total_supply: u64,            // 8
    pub supply_in_position: u64,      // 8
    pub holders: u64,                 // 8
    pub dividends: bool,              // 1
    pub dividend_payment_period: i64, // 8
    pub date_to_go_public: i64,       // 8
    pub price_to_go_public: u64,      // 8
    pub current_offers: u32,          // 4
} // total = 344

#[account]
pub struct HolderAccount {
    pub bump_original: u8,     // 1
    pub participation: u64,    // 8
    pub holder_pubkey: Pubkey, // 32
} // total = 41

#[account]
pub struct SellOrBuyAccount {
    pub bump_original: u8,            // 1
    pub sell_or_buy_amount: Vec<u64>, // 4 + 8
    pub price: Vec<u64>,              // 4 + 8
    pub pubkey: Pubkey,               // 32
    pub len: u64,                     // 8
} // total = 65

impl SellOrBuyAccount {
    pub const SIZE: usize = 1 + 4 + 8 + 4 + 8 + 32 + 8;

    pub fn sub_len(&mut self, amount: u64) {
        self.len -= amount;
    }

    pub fn add_len(&mut self, amount: u64) {
        self.len += amount;
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn init_sell_or_buy_amount(&mut self) {
        self.sell_or_buy_amount = [].to_vec();
    }

    pub fn init_price(&mut self) {
        self.price = [].to_vec();
    }

    pub fn set_pubkey(&mut self, pubkey: Pubkey) {
        self.pubkey = pubkey;
    }

    pub fn set_len(&mut self, len: u64) {
        self.len = len;
    }
}

impl HolderAccount {
    pub const SIZE: usize = 1 + 8 + 32;

    pub fn add_participation(&mut self, amount: u64) {
        self.participation += amount;
    }

    pub fn sub_participation(&mut self, amount: u64) {
        self.participation -= amount;
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_participation(&mut self, amount: u64) {
        self.participation = amount;
    }

    pub fn init_participation(&mut self) {
        self.participation = 0;
    }

    pub fn set_holder_pubkey(&mut self, pubkey: Pubkey) {
        self.holder_pubkey = pubkey;
    }
}

impl StockAccount {
    pub const SIZE: usize = 1 + 32 + 4 + 50 + 4 + 200 + 8 + 8 + 1 + 8 + 8 + 8 + 4 + 8;

    pub fn sub_current_offers(&mut self) {
        self.current_offers -= 1;
    }

    pub fn add_current_offers(&mut self) {
        self.current_offers += 1;
    }

    pub fn add_holders(&mut self) {
        self.holders += 1;
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_pubkey(&mut self, pubkey: Pubkey) {
        self.pubkey_original = pubkey;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_total_supply(&mut self, supply: u64) {
        self.total_supply = supply;
    }

    pub fn set_supply_in_position(&mut self, supply: u64) {
        self.supply_in_position = supply;
    }

    pub fn sub_supply_in_position(&mut self, amount: u64) {
        self.supply_in_position -= amount;
    }

    pub fn init_holders(&mut self) {
        self.holders = 1;
    }

    pub fn set_dividends(&mut self, dividends: bool) {
        self.dividends = dividends;
    }

    pub fn set_dividend_payment_period(&mut self, dividend_payment_period: i64) {
        self.dividend_payment_period = dividend_payment_period;
    }

    pub fn set_date_to_go_public(&mut self, date_to_go_public: i64) {
        self.date_to_go_public = date_to_go_public;
    }

    pub fn set_price_to_go_public(&mut self, price_to_go_public: u64) {
        self.price_to_go_public = price_to_go_public;
    }
}

impl SystemExchangeAccount {
    pub const SIZE: usize = 1 + 4 + 8 + 8 + 8;

    pub fn add_historical_exchanges(&mut self) {
        self.historical_exchanges += 1;
    }

    pub fn add_total_offers(&mut self) {
        self.total_offers += 1;
    }

    pub fn add_total_holders(&mut self) {
        self.total_holders += 1;
    }

    pub fn sub_total_offers(&mut self) {
        self.total_offers -= 1;
    }

    pub fn add_total_stock_companies(&mut self) {
        self.total_stock_companies += 1;
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump_original = bump;
    }
    pub fn init_stock_companies(&mut self) {
        self.total_stock_companies = 0;
    }

    pub fn init_historical_exchanges(&mut self) {
        self.historical_exchanges = 0;
    }

    pub fn init_total_holders(&mut self) {
        self.total_holders = 0;
    }

    pub fn init_total_offers(&mut self) {
        self.total_offers = 0;
    }
}
