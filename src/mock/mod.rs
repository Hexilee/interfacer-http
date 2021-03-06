//! mock module for tests.
//! need `mock` feature.

#[doc(inline)]
pub use error::{Error, Result};

#[doc(inline)]
pub use client::Client;

mod client;
mod error;
