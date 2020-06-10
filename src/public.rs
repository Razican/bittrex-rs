//! Public API methods.

use crate::{
    types::{CurrencyInfo, MarketInfo, MarketSummary, Order, OrderBook, TickerInfo},
    ApiResult, Client, API_URL,
};
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;

/// Public API methods.
impl Client {
    /// Used to get the open and available trading markets at Bittrex along with other meta data.
    pub fn get_markets() -> Result<Vec<MarketInfo>> {
        /// URL for the `get_markets` endpoint.
        static URL: Lazy<String> = Lazy::new(|| API_URL.to_string() + "/public/getmarkets");

        let response = ureq::get(&URL).call();
        let result: ApiResult<Vec<MarketInfo>> = response.into_json_deserialize()?;
        result.into_result()
    }

    /// Used to get all supported currencies at Bittrex along with other meta data.
    pub fn get_currencies() -> Result<Vec<CurrencyInfo>> {
        /// URL for the `get_currencies` endpoint.
        static URL: Lazy<String> = Lazy::new(|| API_URL.to_string() + "/public/getcurrencies");

        let response = ureq::get(&URL).call();
        let result: ApiResult<Vec<CurrencyInfo>> = response.into_json_deserialize()?;
        result.into_result()
    }

    /// Used to get the current tick values for a market.
    pub fn get_ticker<S: AsRef<str>>(market: S) -> Result<TickerInfo> {
        /// URL for the `get_currencies` endpoint.
        static URL: Lazy<String> = Lazy::new(|| API_URL.to_string() + "/public/getticker");

        let mut req = ureq::get(&URL);
        let _ = req.query("market", market.as_ref());

        let response = req.call();
        let result: ApiResult<TickerInfo> = response.into_json_deserialize()?;
        result.into_result()
    }

    /// Used to get the last 24 hour summary of all active exchanges.
    pub fn get_market_summaries() -> Result<Vec<MarketSummary>> {
        /// URL for the `get_currencies` endpoint.
        static URL: Lazy<String> = Lazy::new(|| API_URL.to_string() + "/public/getmarketsummaries");

        let response = ureq::get(&URL).call();
        let result: ApiResult<Vec<MarketSummary>> = response.into_json_deserialize()?;
        result.into_result()
    }

    /// Used to get the last 24 hour summary of the given market.
    pub fn get_market_summary<S: AsRef<str>>(market: S) -> Result<MarketSummary> {
        /// URL for the `get_currencies` endpoint.
        static URL: Lazy<String> = Lazy::new(|| API_URL.to_string() + "/public/getmarketsummary");

        let mut req = ureq::get(&URL);
        let _ = req.query("market", market.as_ref());

        let response = req.call();
        let result: ApiResult<Vec<MarketSummary>> = response.into_json_deserialize()?;
        result.into_result().and_then(|arr| {
            arr.into_iter()
                .next()
                .ok_or_else(|| anyhow!("api response did not contain the market summary"))
        })
    }

    /// Used to get retrieve the order book for a given market.
    ///
    /// **Panics if the depth is bigger than 50.**
    pub fn get_order_book<S: AsRef<str>>(
        market: S,
        order_type: OrderBookType,
        depth: u8,
    ) -> Result<OrderBook> {
        /// URL for the `get_currencies` endpoint.
        static URL: Lazy<String> = Lazy::new(|| API_URL.to_string() + "/public/getorderbook");

        assert!(depth <= 50, "order book depth must be between 0 and 50");

        let mut req = ureq::get(&URL);
        let _ = req
            .query("market", market.as_ref())
            .query("type", order_type.as_str())
            .query("depth", &depth.to_string());

        let response = req.call();

        match order_type {
            OrderBookType::Buy => {
                let result: ApiResult<Box<[Order]>> = response.into_json_deserialize()?;
                Ok(OrderBook::new(result.into_result()?, Vec::new()))
            }
            OrderBookType::Sell => {
                let result: ApiResult<Box<[Order]>> = response.into_json_deserialize()?;
                Ok(OrderBook::new(Vec::new(), result.into_result()?))
            }
            OrderBookType::Both => {
                let result: ApiResult<OrderBook> = response.into_json_deserialize()?;
                result.into_result()
            }
        }
    }

    /// Used to retrieve the latest trades that have occurred for a specific market.
    pub fn get_market_history<S: AsRef<str>>(market: S) -> Result<MarketSummary> {
        /// URL for the `get_currencies` endpoint.
        static URL: Lazy<String> = Lazy::new(|| API_URL.to_string() + "/public/getmarketsummary");

        let mut req = ureq::get(&URL);
        let _ = req.query("market", market.as_ref());

        let response = req.call();
        let result: ApiResult<Vec<MarketSummary>> = response.into_json_deserialize()?;
        result.into_result().and_then(|arr| {
            arr.into_iter()
                .next()
                .ok_or_else(|| anyhow!("api response did not contain the market summary"))
        })
    }
}

/// Enumeration representing the type of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderBookType {
    /// Buying order.
    Buy,
    /// Selling order.
    Sell,
    /// Both, buying and selling orders.
    Both,
}

impl OrderBookType {
    /// Gets a string representation of the order book type.
    fn as_str(&self) -> &str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
            Self::Both => "both",
        }
    }
}
