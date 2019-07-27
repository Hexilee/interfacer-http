pub mod fail;
pub mod polyfill;

use crate::content_type::ContentType;
use crate::StdResult;

// TODO: use T: AsyncRead as type of data
// TODO: declare content_type as generics when const generics is stable
pub trait FromContent: Sized {
    type Err;
    fn from_content(data: Vec<u8>, content_type: &ContentType) -> StdResult<Self, Self::Err>;
}

pub trait IntoStruct<T: Sized> {
    type Err;
    fn into_struct(self, content_type: &ContentType) -> StdResult<T, Self::Err>;
}

impl<T: FromContent> IntoStruct<T> for Vec<u8> {
    type Err = <T as FromContent>::Err;
    fn into_struct(self, content_type: &ContentType) -> Result<T, Self::Err> {
        <T as FromContent>::from_content(self, content_type)
    }
}

// TODO: use T: AsyncRead as type of ret
// TODO: declare content_type as generics when const generics is stable
pub trait ToContent {
    type Err;
    fn to_content(&self, content_type: &ContentType) -> StdResult<Vec<u8>, Self::Err>;
}

#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;

#[cfg(feature = "unhtml-html")]
mod unhtml_support;

mod encode;
