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
pub mod error;
pub mod types;

use reqwest::Url;
use serde::Deserialize;

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
}

impl Client {
    /// Creates a new client with default configuration.
    pub fn new() -> Result<Client> {
        Ok(Client { inner: reqwest::Client::new()? })
    }

    /// Hash the given URI with the given API secret.
    #[allow(unused)]
    fn hash_uri<S: AsRef<str>>(&self, url: &Url, api_secret: S) -> String {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        use hex::ToHex;

        let mut hmac = Hmac::<Sha256>::new(api_secret.as_ref().as_bytes());
        hmac.input(url.as_str().as_ref());

        hmac.result().code().to_hex()
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
