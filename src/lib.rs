//! Bittrex API.

#![forbid(anonymous_parameters, unsafe_code)]
#![warn(clippy::pedantic)]
#![deny(
    clippy::all,
    variant_size_differences,
    unused_results,
    unused,
    unused_qualifications,
    unused_import_braces,
    unused_lifetimes,
    unreachable_pub,
    trivial_numeric_casts,
    trivial_casts,
    missing_docs,
    rustdoc,
    missing_debug_implementations,
    missing_copy_implementations,
    deprecated_in_future,
    meta_variable_misuse,
    non_ascii_idents,
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    //warnings
)]
#![allow(clippy::must_use_candidate, clippy::missing_errors_doc)]

mod private;
mod public;
pub mod types;

pub use crate::public::OrderBookType;
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use serde::Deserialize;
use ureq::Request;

/// API URL.
static API_URL: &str = "https://bittrex.com/api/v1.1/";

/// Bittrex API client.
#[derive(Debug, Default, Clone)]
pub struct Client {
    /// API key.
    api_key: Option<String>,
    /// API secret.
    api_secret: Option<String>,
}

impl Client {
    /// Logs the client in for future private method access.
    ///
    /// Note: This will not perform any request to Bittrex, it will only store the API key and the
    /// secret for future use.
    pub fn login<K: Into<String>, S: Into<String>>(&mut self, api_key: K, api_secret: S) {
        self.api_key = Some(api_key.into());
        self.api_secret = Some(api_secret.into());
    }

    /// Appends the login information to the query string.
    ///
    /// **Note: it will panic if not logged in.**
    fn append_login(&self, req: &mut Request) {
        let api_key = self.api_key.as_ref().expect("no API key provided");
        let nonce = Utc::now().timestamp();
        let _ = req
            .query("apikey", api_key)
            .query("nonce", &nonce.to_string());
    }

    /// Gets the headers for the given URL.
    ///
    /// **Note: it will panic if not logged in.**
    fn set_headers(&self, req: &mut Request) -> Result<()> {
        let url = req.get_url().to_string() + &req.get_query().unwrap();
        let hash = self.hash_uri(&url)?;
        let _ = req.set("apisign", &hash);

        Ok(())
    }

    /// Hash the given URI with the logged in API secret.
    ///
    /// **Note: it will panic if not logged in.**
    fn hash_uri(&self, url: &str) -> Result<String> {
        use hex::ToHex;
        use hmac::{Hmac, Mac, NewMac};
        use sha2::Sha512;

        let api_secret = self
            .api_secret
            .as_ref()
            .expect("the client was not logged in");
        let mut hmac = Hmac::<Sha512>::new_varkey(api_secret.as_bytes())
            .map_err(|_| anyhow!("invalid key length"))?;
        hmac.update(url.as_bytes());

        Ok(hmac.finalize().into_bytes().encode_hex())
    }
}

/// API result structure.
#[derive(Debug, Clone, Deserialize)]
struct ApiResult<R> {
    /// Wether the API call was successful.
    success: bool,
    /// The message returned in the API call.
    message: String,
    /// The result of the API call.
    result: Option<R>,
}

impl<R> ApiResult<R>
where
    R: for<'r> Deserialize<'r>,
{
    /// Converts the API result to a generic result, returning an error if the request was not
    /// successful.
    fn into_result(self) -> Result<R> {
        if self.success {
            if let Some(r) = self.result {
                Ok(r)
            } else {
                bail!("invalid result in success response")
            }
        } else {
            bail!("{}", self.message)
        }
    }
}
