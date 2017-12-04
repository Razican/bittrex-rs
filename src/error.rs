//! Errors for Bittrex API.

#![allow(variant_size_differences)]

error_chain!{
    foreign_links {
        Fmt(::std::fmt::Error) #[doc = "Format error"];
        ReqwestUrl(::reqwest::UrlError) #[doc = "Reqwest URL parsing error"];
        Reqwest(::reqwest::Error) #[doc = "Reqwest error"];
        // HmacInvalidKeyLength(::hmac::crypto_mac::InvalidKeyLength)
        //     #[doc = "Invalid HMAC key length"];
    }

    errors {
        /// Error interacting with the API.
        Api(error: String) {
            description("error in the API")
            display("error in the API: {}", error)
        }

        /// Error result received from the API.
        Result(error: String) {
            description("the API returned an error")
            display("the API returned an error: {}", error)
        }
    }
}
