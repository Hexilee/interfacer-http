#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;
use interfacer_http::{FromContentDerive, ToContentDerive};
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, FromContent, ToContent, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}
