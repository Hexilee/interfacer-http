#![feature(const_generics)]

pub use content::{FromContent, IntoContent};
pub use content_type::ContentType;
use core::result::Result as StdResult;
pub use fail::{RequestFail, Result};
pub use interfacer_http_attribute::http_service;
pub use interfacer_http_util::*;
pub use service::{HttpClient, HttpService};

mod content;
mod content_type;
mod fail;
mod service;
