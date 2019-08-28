#[doc(hidden)]
pub mod polyfill;
pub use error::{FromContentError, ToContentError};
pub use mime_ext::MimeExt;

#[cfg(feature = "encoding")]
mod encoding;
mod error;
mod mime_ext;
#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;
#[cfg(feature = "unhtml-html")]
mod unhtml_support;

use crate::mime::Mime;

// TODO: use T: AsyncRead as type of data
// TODO: declare mime as generics when const generics is stable
pub trait FromContent: Sized {
    fn from_content(data: Vec<u8>, content_type: &Mime) -> Result<Self, FromContentError>;
}

// TODO: use T: AsyncRead as type of ret
// TODO: declare mime as generics when const generics is stable
pub trait ToContent {
    fn to_content(&self, content_type: &Mime) -> Result<Vec<u8>, ToContentError>;
}

pub trait ContentInto<T: Sized> {
    fn content_into(self, content_type: &Mime) -> Result<T, FromContentError>;
}

impl<T: FromContent> ContentInto<T> for Vec<u8> {
    fn content_into(self, content_type: &Mime) -> Result<T, FromContentError> {
        <T as FromContent>::from_content(self, content_type)
    }
}

// TODO: support more special build-in types.
mod impls {
    use crate::mime::Mime;
    use crate::{FromContent, FromContentError, ToContent, ToContentError};
    impl FromContent for () {
        fn from_content(_data: Vec<u8>, _content_type: &Mime) -> Result<Self, FromContentError> {
            Ok(())
        }
    }

    impl ToContent for () {
        fn to_content(&self, _content_type: &Mime) -> Result<Vec<u8>, ToContentError> {
            Ok(Vec::new())
        }
    }
}
