//! Private API methods.
//!
//! **Note: All this methods will panic if the client is not logged in.**

use crate::{types::*, ApiResult, Client, API_URL};
use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::Url;

/// Private API methods.
///
/// **Note: All this methods will panic if the client is not logged in.**
impl Client {
    /// Gets the balances of the Bittrex account.
    ///
    /// **Note: it will panic if not logged in.**
    pub fn get_balances(&self) -> Result<Vec<BalanceInfo>> {
        /// URL for the `get_balances` endpoint.
        static URL: Lazy<Url> = Lazy::new(|| API_URL.join("account/getbalances").unwrap());

        let mut url = URL.clone();
        self.append_login(&mut url);

        let headers = self.get_headers(&url)?;
        let response = self.inner.get(url).headers(headers).send()?;
        let result: ApiResult<Vec<BalanceInfo>> = response.json()?;
        result.into_result()
    }
}
