#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;

#[cfg(feature = "derive")]
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "derive")]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct User {
    name: String,
    age: i32,
}
