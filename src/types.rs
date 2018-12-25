//! Types for Bittrex API.

use chrono::NaiveDateTime;

/// Market information structure.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketInfo {
    /// Currency of the market.
    market_currency: String,
    /// Base currency of the market.
    base_currency: String,
    /// Long name of the currency of the market.
    market_currency_long: String,
    /// Long name of the base currency of the market.
    base_currency_long: String,
    /// Minimum trade size.
    min_trade_size: f64,
    /// Market name.
    market_name: String,
    /// Wether the market is active or not.
    is_active: bool,
    /// Creation date and time of the market.
    created: NaiveDateTime,
    /// Notice about the market.
    notice: Option<String>,
    /// Wether the market is sponsored.
    is_sponsored: Option<bool>,
    /// The logo URL for the market.
    logo_url: Option<String>,
}

impl MarketInfo {
    /// Gets the market currency.
    pub fn market_currency(&self) -> &str {
        &self.market_currency
    }

    /// Gets the base currency.
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }

    /// Gets the long name of the market currency.
    pub fn market_currency_long(&self) -> &str {
        &self.market_currency_long
    }

    /// Gets the long name of the base currency.
    pub fn base_currency_long(&self) -> &str {
        &self.base_currency_long
    }

    /// Gets the minimum trade size.
    pub fn min_trade_size(&self) -> f64 {
        self.min_trade_size
    }

    /// Gets the market name.
    pub fn market_name(&self) -> &str {
        &self.market_name
    }

    /// Gets wether the market is active or not.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Gets the market creation time.
    pub fn created(&self) -> NaiveDateTime {
        self.created
    }

    /// Gets the optional notice in the market.
    pub fn notice(&self) -> Option<&String> {
        self.notice.as_ref()
    }

    /// Gets wether the market is sponsored
    pub fn is_sponsored(&self) -> Option<bool> {
        self.is_sponsored
    }

    /// Gets the logo URL for the market.
    pub fn logo_url(&self) -> Option<&String> {
        self.logo_url.as_ref()
    }
}

/// Currency information structure.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrencyInfo {
    /// 3-letter currency code.
    currency: String,
    /// Long currency name.
    currency_long: String,
    /// Minimum number of confirmations to credit the account.
    min_confirmation: u32,
    /// Transaction fee.
    tx_fee: f32,
    /// Wether the currency is active or not.
    is_active: bool,
    /// Coin type string constant.
    coin_type: String,
    /// Optional base address for the coin at Bittrex.
    base_address: Option<String>,
    /// Optional notice about the currency.
    notice: Option<String>,
}

impl CurrencyInfo {
    /// Gets the 3-letter currency code.
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Gets the long currency name.
    pub fn currency_long(&self) -> &str {
        &self.currency_long
    }

    /// Gets the minimum number of confirmations to credit the account.
    pub fn min_confirmation(&self) -> u32 {
        self.min_confirmation
    }

    /// Gets the transaction fee.
    pub fn tx_fee(&self) -> f32 {
        self.tx_fee
    }

    /// Gets wether the currency is active or not.
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Gets the coin type string constant.
    pub fn coin_type(&self) -> &str {
        &self.coin_type
    }

    /// Gets the base address for the coin at Bittrex.
    pub fn base_address(&self) -> Option<&String> {
        self.base_address.as_ref()
    }

    /// Gets the optional notice about the currency.
    pub fn notice(&self) -> Option<&String> {
        self.notice.as_ref()
    }
}

/// Ticker information structure.
#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TickerInfo {
    /// Current bidding/buying price for the market.
    bid: f32,
    /// Current asking/selling price for the market.
    ask: f32,
    /// Last transaction price.
    last: f32,
}

impl TickerInfo {
    /// Gets the current bidding/buying price for the market.
    pub fn bid(&self) -> f32 {
        self.bid
    }

    /// Gets the current asking/selling price for the market.
    pub fn ask(&self) -> f32 {
        self.ask
    }

    /// Gets the last transaction price.
    pub fn last(&self) -> f32 {
        self.last
    }
}

/// Market summary structure
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketSummary {
    /// Name of the market.
    market_name: String,
    /// Highest transaction value in the last 24 hours for the market.
    high: Option<f32>,
    /// Lowest transaction value in the last 24 hours for the market.
    low: Option<f32>,
    /// Last transaction price.
    last: Option<f32>,
    /// Current bidding/buying price.
    bid: Option<f32>,
    /// Current asking/selling price.
    ask: Option<f32>,
    /// Volume of the market.
    volume: Option<f32>,
    /// Base volume of the market.
    base_volume: Option<f32>,
    /// Timestamp of the information.
    time_stamp: NaiveDateTime,
    /// Number of open buying orders.
    open_buy_orders: Option<u32>,
    /// Number of open selling orders.
    open_sell_orders: Option<u32>,
    prev_day: f32,
    /// Market creation time.
    created: NaiveDateTime,
    /// Name to display for the market.
    display_market_name: Option<String>,
}

impl MarketSummary {
    /// Gets the name of the market.
    pub fn market_name(&self) -> &str {
        &self.market_name
    }

    /// Gets the highest transaction value in the last 24 hours for the market.
    pub fn high(&self) -> Option<f32> {
        self.high
    }

    /// Gets the lowest transaction value in the last 24 hours for the market.
    pub fn low(&self) -> Option<f32> {
        self.low
    }

    /// Gets the last transaction price.
    pub fn last(&self) -> Option<f32> {
        self.last
    }

    /// Gets the current bidding/buying price.
    pub fn bid(&self) -> Option<f32> {
        self.bid
    }

    /// Gets the current asking/selling price.
    pub fn ask(&self) -> Option<f32> {
        self.ask
    }

    /// Gets the volume of the market.
    pub fn volume(&self) -> Option<f32> {
        self.volume
    }

    /// Gets the base volume of the market.
    pub fn base_volume(&self) -> Option<f32> {
        self.base_volume
    }

    /// Gets the timestamp of the information.
    pub fn time_stamp(&self) -> NaiveDateTime {
        self.time_stamp
    }

    /// Gets the number of open buying orders.
    pub fn open_buy_orders(&self) -> Option<u32> {
        self.open_buy_orders
    }

    /// Gets the number of open selling orders.
    pub fn open_sell_orders(&self) -> Option<u32> {
        self.open_sell_orders
    }

    /// Gets the price of the previous day.
    pub fn prev_day(&self) -> f32 {
        self.prev_day
    }

    /// Gets the market creation time.
    pub fn created(&self) -> NaiveDateTime {
        self.created
    }

    /// Gets the name to display for the market.
    pub fn display_market_name(&self) -> Option<&String> {
        self.display_market_name.as_ref()
    }
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
#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    /// Quantity being ordered.
    quantity: f32,
    /// Rate/price of the order
    rate: f32,
}

impl Order {
    /// Gets the quantity being ordered.
    pub fn quantity(self) -> f32 {
        self.quantity
    }

    /// Gets the rate/price of the order.
    pub fn rate(self) -> f32 {
        self.rate
    }
}

/// Structure representing a currency balance information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BalanceInfo {
    /// Currency code.
    currency: String,
    /// Balance for the currency.
    balance: f32,
    /// Available balance for the currency.
    available: f32,
    /// Pending balance for the currency.
    pending: f32,
    /// Address of the currency for deposits.
    crypto_address: Option<String>,
    /// Wether a withdrawal has been requested.
    requested: Option<bool>,
    /// UUID of the currency.
    uuid: Option<String>,
}

impl BalanceInfo {
    /// Gets the currency code.
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Gets the balance for the currency.
    pub fn balance(&self) -> f32 {
        self.balance
    }

    /// Gets the available balance for the currency.
    pub fn available(&self) -> f32 {
        self.available
    }

    /// Gets the pending balance for the currency.
    pub fn pending(&self) -> f32 {
        self.pending
    }

    /// Gets the address of the currency for deposits.
    pub fn crypto_address(&self) -> Option<&str> {
        if let Some(s) = &self.crypto_address {
            Some(s.as_str())
        } else {
            None
        }
    }

    /// Gets wether a withdrawal has been requested.
    pub fn requested(&self) -> bool {
        self.requested == Some(true)
    }

    /// Gets the UUID of the currency.
    pub fn uuid(&self) -> Option<&str> {
        if let Some(s) = &self.uuid {
            Some(s.as_str())
        } else {
            None
        }
    }
}
