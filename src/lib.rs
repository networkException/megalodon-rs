#![deny(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # Megalodon
//! The `megalodon` is a client library for Mastodon and Pleroma. It provides REST API and streaming method which uses WebSocket. By using this library, you can take Mastodon and Pleroma with the same interface.
//!
//! ## Making Mastodon request
//! For a request without authentication.
//!
//! ```rust
//! # use megalodon;
//! # use megalodon::error::Error;
//! #
//! # async fn run() -> Result<(), Error> {
//! let client = megalodon::generator(
//!   megalodon::SNS::Mastodon,
//!   String::from("https://fedibird.com"),
//!   None,
//!   None,
//! );
//! let res = client.get_instance().await?;
//! println!("{:#?}", res.json());
//! # Ok(())
//! # }
//! ```
//!
//! ## Making Mastodon request with authentication
//! For a request with authentication.
//!
//! ```rust
//! # use megalodon;
//! # use megalodon::error::Error;
//! #
//! # async fn run() -> Result<(), Error> {
//! let client = megalodon::generator(
//!   megalodon::SNS::Mastodon,
//!   String::from("https://fedibird.com"),
//!   Some(String::from("your access token")),
//!   None,
//! );
//! let res = client.verify_account_credentials().await?;
//! println!("{:#?}", res.json());
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

pub mod default;
pub mod entities;
pub mod error;
pub mod mastodon;
pub mod megalodon;
pub mod oauth;
pub mod pleroma;
pub mod response;
pub mod streaming;

pub use self::megalodon::Megalodon;
pub use streaming::Streaming;

#[derive(Deserialize, Serialize, Debug)]
struct Instance {
    title: String,
    uri: String,
    urls: entities::URLs,
    version: String,
    pleroma: Option<pleroma::entities::instance::PleromaConfig>,
}

/// Detect which SNS the provided URL is. To detect SNS, the URL has to open `/api/v1/instance` or `/api/meta` endpoint.
pub async fn detector(url: &str) -> Result<SNS, error::Error> {
    let client = reqwest::Client::builder().user_agent("megalodon").build()?;
    let res = client
        .get(format!("{}{}", url, "/api/v1/instance"))
        .send()
        .await;

    match res {
        Ok(res) => {
            let obj = res.json::<Instance>().await;
            match obj {
                Ok(json) => {
                    if let Some(_pleroma) = json.pleroma {
                        Ok(SNS::Pleroma)
                    } else {
                        Ok(SNS::Mastodon)
                    }
                }
                Err(err) => Err(err.into()),
            }
        }
        Err(_) => {
            let client = reqwest::Client::new();
            let res = client.post(format!("{}{}", url, "/api/meta")).send().await;
            match res {
                Ok(_) => Ok(SNS::Misskey),
                Err(err) => Err(err.into()),
            }
        }
    }
}

/// Which SNS.
#[derive(Debug, Clone)]
pub enum SNS {
    /// SNS is Mastodon.
    Mastodon,
    /// SNS is Pleroma.
    Pleroma,
    /// SNS is Misskey.
    Misskey,
}

impl fmt::Display for SNS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SNS::Mastodon => write!(f, "mastodon"),
            SNS::Pleroma => write!(f, "pleroma"),
            SNS::Misskey => write!(f, "misskey"),
        }
    }
}

impl FromStr for SNS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mastodon" => Ok(SNS::Mastodon),
            "pleroma" => Ok(SNS::Pleroma),
            "misskey" => Ok(SNS::Misskey),
            &_ => Err(format!("Unknown sns: {}", s)),
        }
    }
}

/// Generate an API client which satisfies megalodon trait.
pub fn generator(
    sns: SNS,
    base_url: String,
    access_token: Option<String>,
    user_agent: Option<String>,
) -> Box<dyn Megalodon + Send + Sync> {
    match sns {
        SNS::Pleroma => {
            let pleroma = pleroma::Pleroma::new(base_url, access_token, user_agent);
            Box::new(pleroma)
        }
        _ => {
            let mastodon = mastodon::Mastodon::new(base_url, access_token, user_agent);
            Box::new(mastodon)
        }
    }
}
