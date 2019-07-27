#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}
