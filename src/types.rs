//! Types for Bittrex API.

use chrono::NaiveDateTime;
use getset::{CopyGetters, Getters};
use serde::Deserialize;

/// Market information structure.
#[derive(Debug, Clone, Deserialize, CopyGetters, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct MarketInfo {
    /// Currency of the market.
    #[get = "pub"]
    market_currency: String,
    /// Base currency of the market.
    #[get = "pub"]
    base_currency: String,
    /// Long name of the currency of the market.
    #[get = "pub"]
    market_currency_long: String,
    /// Long name of the base currency of the market.
    #[get = "pub"]
    base_currency_long: String,
    /// Minimum trade size.
    #[get_copy = "pub"]
    min_trade_size: f64,
    /// Market name.
    #[get = "pub"]
    market_name: String,
    /// Wether the market is active or not.
    #[get_copy = "pub"]
    is_active: bool,
    /// Creation date and time of the market.
    #[get_copy = "pub"]
    created: NaiveDateTime,
    /// Notice about the market.
    #[get = "pub"]
    notice: Option<String>,
    /// Wether the market is sponsored.
    #[get_copy = "pub"]
    is_sponsored: Option<bool>,
    /// The logo URL for the market.
    #[get = "pub"]
    logo_url: Option<String>,
}

/// Currency information structure.
#[derive(Debug, Clone, Deserialize, Getters, CopyGetters)]
#[serde(rename_all = "PascalCase")]
pub struct CurrencyInfo {
    /// 3-letter currency code.
    #[get = "pub"]
    currency: String,
    /// Long currency name.
    #[get = "pub"]
    currency_long: String,
    /// Minimum number of confirmations to credit the account.
    #[get_copy = "pub"]
    min_confirmation: u32,
    /// Transaction fee.
    #[get_copy = "pub"]
    tx_fee: f32,
    /// Wether the currency is active or not.
    #[get_copy = "pub"]
    is_active: bool,
    /// Coin type string constant.
    #[get = "pub"]
    coin_type: String,
    /// Optional base address for the coin at Bittrex.
    #[get = "pub"]
    base_address: Option<String>,
    /// Optional notice about the currency.
    #[get = "pub"]
    notice: Option<String>,
}

/// Ticker information structure.
#[derive(Debug, Copy, Clone, Deserialize, CopyGetters)]
#[serde(rename_all = "PascalCase")]
pub struct TickerInfo {
    /// Current bidding/buying price for the market.
    #[get_copy = "pub"]
    bid: f32,
    /// Current asking/selling price for the market.
    #[get_copy = "pub"]
    ask: f32,
    /// Last transaction price.
    #[get_copy = "pub"]
    last: f32,
}

/// Market summary structure
#[derive(Debug, Clone, Deserialize, CopyGetters, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct MarketSummary {
    /// Name of the market.
    #[get = "pub"]
    market_name: String,
    /// Highest transaction value in the last 24 hours for the market.
    #[get_copy = "pub"]
    high: Option<f32>,
    /// Lowest transaction value in the last 24 hours for the market.
    #[get_copy = "pub"]
    low: Option<f32>,
    /// Last transaction price.
    #[get_copy = "pub"]
    last: Option<f32>,
    /// Current bidding/buying price.
    #[get_copy = "pub"]
    bid: Option<f32>,
    /// Current asking/selling price.
    #[get_copy = "pub"]
    ask: Option<f32>,
    /// Volume of the market.
    #[get_copy = "pub"]
    volume: Option<f32>,
    /// Base volume of the market.
    #[get_copy = "pub"]
    base_volume: Option<f32>,
    /// Timestamp of the information.
    #[get_copy = "pub"]
    time_stamp: NaiveDateTime,
    /// Number of open buying orders.
    #[get_copy = "pub"]
    open_buy_orders: Option<u32>,
    /// Number of open selling orders.
    #[get_copy = "pub"]
    open_sell_orders: Option<u32>,
    /// Tthe price of the previous day.
    #[get_copy = "pub"]
    prev_day: Option<f32>,
    /// Market creation time.
    #[get_copy = "pub"]
    created: NaiveDateTime,
    /// Name to display for the market.
    #[get = "pub"]
    display_market_name: Option<String>,
}

/// Structure representing an order book.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBook {
    /// List of buying orders.
    buy: Box<[Order]>,
    /// List of selling orders.
    sell: Box<[Order]>,
}

impl OrderBook {
    /// Creates a new order book.
    pub(crate) fn new<B, S>(buy: B, sell: S) -> Self
    where
        B: Into<Box<[Order]>>,
        S: Into<Box<[Order]>>,
    {
        Self {
            buy: buy.into(),
            sell: sell.into(),
        }
    }
    /// Gets the list of buying orders.
    pub fn buy(&self) -> &[Order] {
        &self.buy
    }

    /// Gets the list of selling orders.
    pub fn sell(&self) -> &[Order] {
        &self.sell
    }
}

/// Structure representing an order.
#[derive(Debug, Copy, Clone, Deserialize, CopyGetters)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    /// Quantity being ordered.
    #[get_copy = "pub"]
    quantity: f32,
    /// Rate/price of the order
    #[get_copy = "pub"]
    rate: f32,
}

/// Structure representing a currency balance information.
#[derive(Debug, Clone, Deserialize, CopyGetters, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct BalanceInfo {
    /// Currency code.
    #[get = "pub"]
    currency: String,
    /// Balance for the currency.
    #[get_copy = "pub"]
    balance: f32,
    /// Available balance for the currency.
    #[get_copy = "pub"]
    available: f32,
    /// Pending balance for the currency.
    #[get_copy = "pub"]
    pending: f32,
    /// Address of the currency for deposits.
    #[get = "pub"]
    crypto_address: Option<String>,
    /// Wether a withdrawal has been requested.
    #[get_copy = "pub"]
    requested: Option<bool>,
    /// UUID of the currency.
    #[get = "pub"]
    uuid: Option<String>,
}
