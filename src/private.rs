//! Private API methods.
//!
//! **Note: All this methods will panic if the client is not logged in.**

use failure::Error;
use reqwest::Url;

use types::*;
use {ApiResult, Client, API_URL};

/// Private API methods.
///
/// **Note: All this methods will panic if the client is not logged in.**
impl Client {
    /// Gets the balances of the Bittrex account.
    ///
    /// **Note: it will panic if not logged in.**
    pub fn get_balances(&self) -> Result<Vec<BalanceInfo>, Error> {
        lazy_static! {
            /// URL for the `get_balances` endpoint.
            static ref URL: Url = API_URL.join("account/getbalances").unwrap();
        }
        let mut url = URL.clone();
        self.append_login(&mut url);

        let headers = self.get_headers(&url)?;
        let mut response = self.inner.get(url).headers(headers).send()?;
        let result: ApiResult<Vec<BalanceInfo>> = response.json()?;
        result.into_result()
    }
}
