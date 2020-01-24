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
#![allow(clippy::must_use_candidate)]

mod private;
mod public;
pub mod types;

pub use crate::public::OrderBookType;
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Url,
};
use serde::Deserialize;

/// API URL.
static API_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://bittrex.com/api/v1.1/").unwrap());

/// Bittrex API client.
#[derive(Debug)]
pub struct Client {
    /// Inner reqwest client.
    inner: reqwest::blocking::Client,
    /// API key.
    api_key: Option<String>,
    /// API secret.
    api_secret: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            inner: reqwest::blocking::Client::new(),
            api_key: None,
            api_secret: None,
        }
    }
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
    fn append_login(&self, url: &mut Url) {
        let api_key = self.api_key.as_ref().unwrap();
        let nonce = Utc::now().timestamp();
        let _ = url
            .query_pairs_mut()
            .append_pair("apikey", api_key)
            .append_pair("nonce", &format!("{}", nonce));
    }

    /// Gets the headers for the given URL.
    ///
    /// **Note: it will panic if not logged in.**
    fn get_headers(&self, url: &Url) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        let hash = self.hash_uri(url);
        let _ = headers.insert("apisign", HeaderValue::from_str(&hash?)?);
        Ok(headers)
    }

    /// Hash the given URI with the logged in API secret.
    ///
    /// **Note: it will panic if not logged in.**
    fn hash_uri(&self, url: &Url) -> Result<String> {
        use hex::ToHex;
        use hmac::{Hmac, Mac};
        use sha2::Sha512;

        let api_secret = self
            .api_secret
            .as_ref()
            .expect("the client was not logged in");
        let mut hmac = Hmac::<Sha512>::new_varkey(api_secret.as_bytes())
            .map_err(|_| anyhow!("invalid key length"))?;
        hmac.input(url.as_str().as_ref());

        Ok(hmac.result().code().as_slice().encode_hex())
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
