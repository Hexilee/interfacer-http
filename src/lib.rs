#![feature(async_await, trait_alias)]

#[doc(inline)]
pub use content::{
    polyfill, ContentInto, FromContent, FromContentError, MimeExt, ToContent, ToContentError,
};
#[doc(inline)]
pub use error::{Error, Report};
#[doc(inline)]
pub use interfacer_http_attribute::http_service;

pub mod derive {
    #[doc(inline)]
    pub use interfacer_http_attribute::{FromContent, ToContent};
}

#[doc(inline)]
pub use client::{Helper, HttpClient, ResponseError, ResponseExt};
pub use interfacer_http_util::*;

mod client;
mod content;
mod error;
