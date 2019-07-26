use super::fail::{FromContentFail, ToContentFail};

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
mod serde_json_support;
