#![feature(const_generics)]

pub use content::{FromContent, IntoContent};
use core::result::Result as StdResult;
pub use interfacer_http_attribute::http_service;
pub use interfacer_http_util::*;
pub use service::{HttpClient, HttpService};

mod content;
mod service;
