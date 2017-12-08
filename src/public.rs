//! Public API methods.

use reqwest::Url;

use {Client, API_URL, ApiResult};
use error::*;
use types::*;

/// Public API methods.
impl Client {
    /// Used to get the open and available trading markets at Bittrex along with other meta data.
    pub fn get_markets(&self) -> Result<Vec<MarketInfo>> {
        lazy_static! {
            /// URL for the `get_markets` endpoint.
            static ref URL: Url = API_URL.join("public/getmarkets").unwrap();
        }

        let mut response = self.inner.get(URL.clone()).send()?;
        let result: ApiResult<Vec<MarketInfo>> = response.json()?;
        result.into_result()
    }

    /// Used to get all supported currencies at Bittrex along with other meta data.
    pub fn get_currencies(&self) -> Result<Vec<CurrencyInfo>> {
        lazy_static! {
            /// URL for the `get_currencies` endpoint.
            static ref URL: Url = API_URL.join("public/getcurrencies").unwrap();
        }

        let mut response = self.inner.get(URL.clone()).send()?;
        let result: ApiResult<Vec<CurrencyInfo>> = response.json()?;
        result.into_result()
    }

    /// Used to get the current tick values for a market.
    pub fn get_ticker<S: AsRef<str>>(&self, market: S) -> Result<TickerInfo> {
        lazy_static! {
            /// URL for the `get_currencies` endpoint.
            static ref URL: Url = API_URL.join("public/getticker").unwrap();
        }

        let mut url = URL.clone();
        let _ = url.query_pairs_mut().append_pair("market", market.as_ref());

        let mut response = self.inner.get(url).send()?;
        let result: ApiResult<TickerInfo> = response.json()?;
        result.into_result()
    }

    /// Used to get the last 24 hour summary of all active exchanges.
    pub fn get_market_summaries(&self) -> Result<Vec<MarketSummary>> {
        lazy_static! {
            /// URL for the `get_currencies` endpoint.
            static ref URL: Url = API_URL.join("public/getmarketsummaries").unwrap();
        }

        let mut response = self.inner.get(URL.clone()).send()?;
        let result: ApiResult<Vec<MarketSummary>> = response.json()?;
        result.into_result()
    }

    /// Used to get the last 24 hour summary of the given market.
    pub fn get_market_summary<S: AsRef<str>>(&self, market: S) -> Result<MarketSummary> {
        lazy_static! {
            /// URL for the `get_currencies` endpoint.
            static ref URL: Url = API_URL.join("public/getmarketsummary").unwrap();
        }

        let mut url = URL.clone();
        let _ = url.query_pairs_mut().append_pair("market", market.as_ref());

        let mut response = self.inner.get(url.clone()).send()?;
        let result: ApiResult<Vec<MarketSummary>> = response.json()?;
        result.into_result().and_then(|arr| {
            arr.into_iter().next().ok_or_else(|| {
                ErrorKind::Api("api response did not contain the market summary".to_owned()).into()
            })
        })
    }

    /// Used to get retrieve the order book for a given market.
    ///
    /// **Panics if the depth is bigger than 50.**
    pub fn get_order_book<S: AsRef<str>>(
        &self,
        market: S,
        order_type: OrderBookType,
        depth: u8,
    ) -> Result<OrderBook> {
        assert!(depth <= 50, "order book depth must be between 0 and 50");
        lazy_static! {
            /// URL for the `get_currencies` endpoint.
            static ref URL: Url = API_URL.join("public/getorderbook").unwrap();
        }

        let mut url = URL.clone();
        let _ = url.query_pairs_mut()
            .append_pair("market", market.as_ref())
            .append_pair("type", order_type.as_str())
            .append_pair("depth", &format!("{}", depth));

        let mut response = self.inner.get(url.clone()).send()?;

        match order_type {
            OrderBookType::Buy => {
                let result: ApiResult<Box<[Order]>> = response.json()?;
                Ok(OrderBook::new(result.into_result()?, Vec::new()))
            }
            OrderBookType::Sell => {
                let result: ApiResult<Box<[Order]>> = response.json()?;
                Ok(OrderBook::new(Vec::new(), result.into_result()?))
            }
            OrderBookType::Both => {
                let result: ApiResult<OrderBook> = response.json()?;
                result.into_result()
            }
        }
    }

    /// Used to retrieve the latest trades that have occured for a specific market.
    pub fn get_market_history<S: AsRef<str>>(&self, market: S) -> Result<MarketSummary> {
        lazy_static! {
            /// URL for the `get_currencies` endpoint.
            static ref URL: Url = API_URL.join("public/getmarketsummary").unwrap();
        }

        let mut url = URL.clone();
        let _ = url.query_pairs_mut().append_pair("market", market.as_ref());

        let mut response = self.inner.get(url.clone()).send()?;
        let result: ApiResult<Vec<MarketSummary>> = response.json()?;
        result.into_result().and_then(|arr| {
            arr.into_iter().next().ok_or_else(|| {
                ErrorKind::Api("api response did not contain the market summary".to_owned()).into()
            })
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
        match *self {
            OrderBookType::Buy => "buy",
            OrderBookType::Sell => "sell",
            OrderBookType::Both => "both",
        }
    }
}
