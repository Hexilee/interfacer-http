use crate::content_type::ContentType;
use crate::StdResult;

// TODO: use T: AsyncRead as type of data
// TODO: declare content_type as generics when const generics is stable
pub trait FromContent: Sized {
    type Err;
    fn from_content(data: Vec<u8>, content_type: &ContentType) -> StdResult<Self, Self::Err>;
}

// TODO: use T: AsyncRead as type of ret
// TODO: declare content_type as generics when const generics is stable
pub trait ToContent {
    type Err;
    fn to_content(&self, content_type: &ContentType) -> StdResult<Vec<u8>, Self::Err>;
}

#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;

pub mod fail;

mod encode;
