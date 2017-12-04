//! Bittrex API.

// #![forbid(missing_docs, warnings)]
#![deny(deprecated, improper_ctypes, non_shorthand_field_patterns, overflowing_literals,
    plugin_as_library, private_no_mangle_fns, private_no_mangle_statics, stable_features,
    unconditional_recursion, unknown_lints, unused, unused_allocation, unused_attributes,
    unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(missing_docs, trivial_casts, trivial_numeric_casts, unused, unused_extern_crates,
    unused_import_braces, unused_qualifications, unused_results, variant_size_differences)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate sha2;
extern crate hmac;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate hex;
extern crate chrono;

mod public;
mod private;
pub mod error;
pub mod types;

use reqwest::Url;
use reqwest::header::Headers;
use serde::Deserialize;
use chrono::Utc;

use error::*;
pub use public::OrderBookType;

/// API URL.
lazy_static! {
    static ref API_URL: Url = Url::parse("https://bittrex.com/api/v1.1/").unwrap();
}

/// Bittrex API client.
#[derive(Debug)]
pub struct Client {
    /// Inner reqwest client.
    inner: reqwest::Client,
    /// API key.
    api_key: Option<String>,
    /// API secret.
    api_secret: Option<String>,
}

impl Client {
    /// Creates a new client with default configuration.
    pub fn new() -> Result<Client> {
        Ok(Client {
            inner: reqwest::Client::new(),
            api_key: None,
            api_secret: None,
        })
    }

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
        let _ = url.query_pairs_mut()
            .append_pair("apikey", api_key)
            .append_pair("nonce", &format!("{}", nonce));
    }

    /// Gets the headers for the given URL.
    ///
    /// **Note: it will panic if not logged in.**
    fn get_headers(&self, url: &Url) -> Result<Headers> {
        let mut headers = Headers::new();
        let hash = self.hash_uri(url);
        headers.set_raw("apisign", hash?);
        Ok(headers)
    }

    /// Hash the given URI with the logged in API secret.
    ///
    /// **Note: it will panic if not logged in.**
    fn hash_uri(&self, url: &Url) -> Result<String> {
        use hmac::{Hmac, Mac};
        use sha2::Sha512;
        use hex::ToHex;

        let api_secret = self.api_secret.as_ref().expect(
            "the client was not logged in",
        );
        let mut hmac = Hmac::<Sha512>::new(api_secret.as_bytes()).map_err(|_| {
            Error::from("invalid key length")
        })?;
        hmac.input(url.as_str().as_ref());

        let mut res = String::new();
        hmac.result().code().as_slice().write_hex(&mut res)?;
        Ok(res)
    }
}

/// API result structure.
#[derive(Debug, Clone, Deserialize)]
struct ApiResult<R> {
    /// Wether the API call was successfull.
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
                Err(
                    ErrorKind::Api(String::from("invalid result in success response")).into(),
                )
            }
        } else {
            Err(ErrorKind::Result(self.message).into())
        }
    }
}
