#![feature(trait_alias, specialization)]

pub extern crate cookie;
pub extern crate http;
pub extern crate mime;
pub extern crate url;

pub use async_trait::async_trait;

#[cfg(feature = "mock")]
pub mod mock;

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

mod client;
mod content;
mod error;
