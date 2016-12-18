
//!  A wrapper library for SongKick API
//!
//!  For API Term of Use Agreement see [here](https://www.songkick.com/developer/api-terms-of-use)
//!
//! # Examples
//!
//! ```rust,no_run
//! use songkick::{SongKick};
//! use songkick::resources::Artist;
//! use songkick::endpoints::{SkEndpoint};
//!
//! let sk = SongKick::new("API_KEY");
//!
//! // RadioHead ID
//! let artists : Vec<Artist> = sk.artist.get(253846)
//! .and_then(|res| Ok(res.collect()))
//! .expect("Failed to fetch artist with id");
//! assert_eq!(1,artists.len());
//!
//! ```
//#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

extern crate hyper;
extern crate serde;
#[macro_use]
extern crate url;
extern crate serde_json;


mod client;
mod result;
mod util;
pub mod options;
pub mod error;
pub mod resources;
pub mod endpoints;

pub use client::SongKick as SongKick;

use error::SkError;

pub type SkResult<T> = Result<T, SkError>;