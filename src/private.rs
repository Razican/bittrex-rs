//! Private API methods.
//!
//! **Note: All this methods will panic if the client is not logged in.**

use crate::{types::BalanceInfo, ApiResult, Client, API_URL};
use anyhow::Result;
use once_cell::sync::Lazy;

/// Private API methods.
///
/// **Note: All this methods will panic if the client is not logged in.**
impl Client {
    /// Gets the balances of the Bittrex account.
    ///
    /// **Note: it will panic if not logged in.**
    pub fn get_balances(&self) -> Result<Vec<BalanceInfo>> {
        /// URL for the `get_balances` endpoint.
        static URL: Lazy<String> = Lazy::new(|| API_URL.to_string() + "/account/getbalances");

        let mut req = ureq::get(&URL);
        self.append_login(&mut req);
        self.set_headers(&mut req)?;

        let response = req.call();
        let result: ApiResult<Vec<BalanceInfo>> = response.into_json_deserialize()?;
        result.into_result()
    }
}
