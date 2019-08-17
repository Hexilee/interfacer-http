//! ## NEVER USE THIS
//!
//! This is a polyfill, used for derive `ToContent` and `FromContent`. It will be removed when const generics is stable.
//!
//! ## Why is const generics required?
//! This is a polyfill module, as const generics is unstable and full of bugs. `ToContent` and `FromContent` cannot have generic constant parameter `CONTENT_TYPE`.
//! Implementing `FromContent` for multiple generic `T` causes conflicting implementation.
//!
//! ```ignore
//! impl<T: DeserializeOwned> FromContent for T {};
//! impl<T: FromHtml> FromContent for T {};
//! // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation
//! ```
//! Using const generics, `FromContent` can be defined as
//!
//! ```ignore
//! trait FromContent<const CONTENT_TYPE: &str> {}
//! ```
//!
//! Then we can
//!
//! ```ignore
//! impl<T: DeserializeOwned> FromContent<"application/json"> for T {};
//! impl<T: FromHtml> FromContent<"text/html"> for T {};
//! // no conflict
//! ```
//!

#[allow(unused_imports)]
use crate::mime::Mime;

#[allow(unused_macros)]
macro_rules! define_from_content {
    ($trait_name:ident) => {
        pub trait $trait_name: Sized {
            type Err;
            fn _from_content(data: Vec<u8>, content_type: &Mime) -> Result<Self, Self::Err>;
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_to_content {
    ($trait_name:ident) => {
        pub trait $trait_name: Sized {
            type Err;
            fn _to_content(&self, content_type: &Mime) -> Result<Vec<u8>, Self::Err>;
        }
    };
}

#[cfg(any(feature = "serde-base", feature = "serde-full"))]
define_from_content!(FromContentSerde);

#[cfg(any(feature = "serde-base", feature = "serde-full"))]
define_to_content!(ToContentSerde);

#[cfg(feature = "unhtml-html")]
define_from_content!(FromContentHtml);
