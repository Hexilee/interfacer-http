#![feature(trait_alias, specialization)]

#[doc(inline)]
pub use content::{
    polyfill, ContentInto, FromContent, FromContentError, MimeExt, ToContent, ToContentError,
};
#[doc(inline)]
pub use error::{Error, Unexpected};
#[doc(inline)]
pub use interfacer_http_attribute::http_service;

#[doc(inline)]
pub use interfacer_http_attribute::{FromContent, ToContent};

#[doc(inline)]
pub use client::{CookieError, Helper, HttpClient, ResponseExt};
pub use interfacer_http_util::*;

mod client;
mod content;
mod error;
