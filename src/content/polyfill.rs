//! ## BE CAUTIOUS TO USE THIS
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

#[cfg(feature = "unhtml-html")]
use crate::mime::Mime;

#[cfg(feature = "unhtml-html")]
use crate::FromContentError;

#[macro_export]
macro_rules! define_from_content {
    ($trait_name:ident) => {
        pub trait $trait_name: Sized {
            fn _from_content(data: Vec<u8>, content_type: &Mime) -> Result<Self, FromContentError>;
        }
    };
}

#[macro_export]
macro_rules! define_to_content {
    ($trait_name:ident) => {
        pub trait $trait_name: Sized {
            fn _to_content(&self, content_type: &Mime) -> Result<Vec<u8>, ToContentError>;
        }
    };
}

#[cfg(feature = "unhtml-html")]
define_from_content!(FromContentHtml);
