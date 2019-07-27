#![feature(decl_macro)]

#[doc(inline)]
pub use content::{fail::*, polyfill, FromContent, ToContent};
#[doc(inline)]
pub use content_type::ContentType;
use core::result::Result as StdResult;
#[doc(inline)]
pub use fail::{define_from, RequestFail, Result};
pub use failure::Fail;
#[doc(inline)]
pub use interfacer_http_attribute::http_service;
#[doc(inline)]
pub use interfacer_http_util::*;
#[doc(inline)]
pub use service::{HttpClient, HttpService};

mod content;
mod content_type;
mod fail;
mod service;
