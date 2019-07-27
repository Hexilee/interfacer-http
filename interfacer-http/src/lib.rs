#![feature(decl_macro)]

pub use content::{fail::*, polyfill, FromContent, ToContent};
pub use content_type::ContentType;
use core::result::Result as StdResult;
pub use fail::{define_from, RequestFail, Result};
pub use failure::Fail;
pub use interfacer_http_attribute::{
    http_service, FromContent as FromContentDerive, ToContent as ToContentDerive,
};
pub use interfacer_http_util::*;
pub use service::{HttpClient, HttpService};

mod content;
mod content_type;
mod fail;
mod service;
