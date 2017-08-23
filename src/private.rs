//! Private API methods.
//!
//! **Note: All this methods will panic if the client is not logged in.**

use reqwest::Url;

use {Client, API_URL, ApiResult};
use error::*;
use types::*;

/// Private API methods.
///
/// **Note: All this methods will panic if the client is not logged in.**
impl Client {
    /// Gets the balances of the Bittrex account.
    ///
    /// **Note: it will panic if not logged in.**
    pub fn get_balances(&self) -> Result<Box<[BalanceInfo]>> {
        lazy_static! {
            /// URL for the `get_balances` endpoint.
            static ref URL: Url = API_URL.join("account/getbalances").unwrap();
        }
        let mut url = URL.clone();
        self.append_login(&mut url);

        let headers = self.get_headers(&url);
        let mut response = self.inner.get(url)?.headers(headers).send()?;
        let result: ApiResult<Box<[BalanceInfo]>> = response.json()?;
        result.into_result()
    }
}
