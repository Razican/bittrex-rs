//! Integration tests module.

#![forbid(anonymous_parameters, unsafe_code)]
#![warn(clippy::pedantic)]
#![deny(
    clippy::all,
    variant_size_differences,
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

use bittrex::Client;
use chrono::{Datelike, Timelike};
use dotenv::dotenv;
use std::env;

/// Tests getting all the markets.
#[test]
fn it_get_markets() {
    Client::get_markets().unwrap();
}

/// Tests that the date/time parsing from the API is done properly.
#[test]
fn it_date_time_parsing() {
    let markets = Client::get_markets().unwrap();

    for market in markets {
        if market.market_name() == "BTC-LTC" {
            // "2014-02-13T00:00:00"
            let creation = market.created();

            assert_eq!(creation.year(), 2014);
            assert_eq!(creation.month(), 2);
            assert_eq!(creation.day(), 13);

            assert_eq!(creation.hour(), 0);
            assert_eq!(creation.minute(), 0);
            assert_eq!(creation.second(), 0);
            assert_eq!(creation.nanosecond(), 0);
        } else if market.market_currency() == "BTC-BYC" {
            // "2014-11-03T19:02:45.96"
            let creation = market.created();

            assert_eq!(creation.year(), 2014);
            assert_eq!(creation.month(), 11);
            assert_eq!(creation.day(), 3);

            assert_eq!(creation.hour(), 19);
            assert_eq!(creation.minute(), 2);
            assert_eq!(creation.second(), 45);
            assert_eq!(creation.nanosecond(), 960_000_000);
        }
    }
}

/// Tests getting all currencies.
#[test]
fn it_get_currencies() {
    Client::get_currencies().unwrap();
}

/// Tests getting the ticker information for 3 valid and 1 invalid markets.
#[test]
fn it_get_ticker() {
    Client::get_ticker("BTC-LTC").unwrap();
    Client::get_ticker("BTC-ETH").unwrap();
    Client::get_ticker("BTC-ZEC").unwrap();

    assert!(Client::get_ticker("Invalid market").is_err());
}

/// Tests getting all the market summaries.
#[test]
fn it_get_market_summaries() {
    Client::get_market_summaries().unwrap();
}

/// Tests 3 valid and 1 invalid market summary for an expected response.
#[test]
fn it_get_market_summary() {
    Client::get_market_summary("BTC-LTC").unwrap();
    Client::get_market_summary("BTC-ETH").unwrap();
    Client::get_market_summary("BTC-ZEC").unwrap();

    assert!(Client::get_market_summary("Invalid market").is_err());
}

/// Tests 3 valid markets and an invalid market for an expected response.
#[test]
fn it_get_order_book() {
    use bittrex::OrderBookType;

    Client::get_order_book("BTC-LTC", OrderBookType::Buy, 20).unwrap();
    Client::get_order_book("BTC-ETH", OrderBookType::Sell, 0).unwrap();
    Client::get_order_book("BTC-ZEC", OrderBookType::Both, 50).unwrap();

    assert!(Client::get_order_book("Invalid market", OrderBookType::Both, 50).is_err());
}

/// Tests an invalid order book depth, that should panic.
#[test]
#[should_panic]
fn it_get_order_book_panic() {
    use bittrex::OrderBookType;

    Client::get_order_book("BTC-LTC", OrderBookType::Both, 100).unwrap();
}

/// Test get balances from account.
#[test]
#[ignore]
fn it_get_balances() {
    dotenv().ok().unwrap();

    let api_key = env::var("API_KEY").unwrap();
    let api_secret = env::var("API_SECRET").unwrap();

    let mut client = Client::default();
    client.login(api_key, api_secret);

    client.get_balances().unwrap();
}
