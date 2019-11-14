pub mod error;

#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;

#[cfg(feature = "unhtml-html")]
mod unhtml_support;

// TODO: use T: AsyncRead as type of data
/// deserialize from response body by `Content-Type` of `Response`.
/// target type of `http_service` method should implement FromContent.
pub trait FromContent<T>: Sized {
    type Err: std::error::Error;
    fn from_content(data: Vec<u8>) -> Result<Self, Self::Err>;
}

// TODO: use T: AsyncRead as type of ret
/// serialize into request body by `Content-Type` of `Request`.
/// body of `http_service` method should implement ToContent.
pub trait ToContent<T> {
    type Err: std::error::Error;
    fn to_content(&self) -> Result<Vec<u8>, Self::Err>;
}

/// Wrapped trait of `FromContent` for generic return type deduction.
pub trait ContentInto<M, T: Sized> {
    type Err: std::error::Error;
    fn content_into(self) -> Result<T, Self::Err>;
}

impl<M, T: FromContent<M>> ContentInto<M, T> for Vec<u8> {
    type Err = T::Err;
    fn content_into(self) -> Result<T, Self::Err> {
        T::from_content(self)
    }
}

// TODO: support more special build-in types.
mod impls {
    use crate::{FromContent, FromContentError, ToContent, ToContentError};
    impl<T> FromContent<T> for () {
        type Err = FromContentError;
        fn from_content(_data: Vec<u8>) -> Result<Self, Self::Err> {
            Ok(())
        }
    }

    impl<T> ToContent<T> for () {
        type Err = ToContentError;
        fn to_content(&self) -> Result<Vec<u8>, Self::Err> {
            Ok(Vec::new())
        }
    }
}
