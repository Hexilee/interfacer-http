#[doc(hidden)]
pub mod polyfill;

#[cfg(feature = "encoding")]
mod encoding;
mod error;
mod mime;
#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;
#[cfg(feature = "unhtml-html")]
mod unhtml_support;

use crate::mime::Mime;
pub use error::{FromContentError, ToContentError};
pub use mime::MimeExt;

// TODO: use T: AsyncRead as type of data
// TODO: declare mime as generics when const generics is stable
pub trait FromContent: Sized {
    type Err: Into<FromContentError>;
    fn from_content(data: Vec<u8>, content_type: &Mime) -> Result<Self, Self::Err>;
}

// TODO: use T: AsyncRead as type of ret
// TODO: declare mime as generics when const generics is stable
pub trait ToContent {
    type Err: Into<ToContentError>;
    fn to_content(&self, content_type: &Mime) -> Result<Vec<u8>, Self::Err>;
    fn to_content_map_err(&self, content_type: &Mime) -> Result<Vec<u8>, ToContentError> {
        self.to_content(content_type).map_err(Into::into)
    }
}

pub trait ContentInto<T: Sized> {
    fn content_into(self, content_type: &Mime) -> Result<T, FromContentError>;
}

impl<T: FromContent> ContentInto<T> for Vec<u8> {
    fn content_into(self, content_type: &Mime) -> Result<T, FromContentError> {
        <T as FromContent>::from_content(self, content_type).map_err(Into::into)
    }
}

// TODO: support more build-in types.
mod impls {
    use crate::mime::Mime;
    use crate::{FromContent, FromContentError, ToContent, ToContentError};
    impl FromContent for () {
        type Err = FromContentError;
        fn from_content(_data: Vec<u8>, _content_type: &Mime) -> Result<Self, Self::Err> {
            Ok(())
        }
    }

    impl ToContent for () {
        type Err = ToContentError;
        fn to_content(&self, _content_type: &Mime) -> Result<Vec<u8>, Self::Err> {
            Ok(Vec::new())
        }
    }
}
