use crate::StdResult;

// TODO: use T: AsyncRead as type of data
// TODO: declare charset as generics when Option is supported by const generics
pub trait FromContent<const CONTENT_TYPE: &'static str>: Sized {
    type Err;
    fn from_content(data: &[u8], encode: Option<&str>) -> StdResult<Self, Self::Err>;
}

// TODO: use T: AsyncRead as type of ret
// TODO: declare charset as generics when Option is supported by const generics
pub trait ToContent<const CONTENT_TYPE: &'static str> {
    type Err;
    fn to_content(&self, encode: Option<&str>) -> StdResult<Vec<u8>, Self::Err>;
}

mod serde_support;

pub mod fail;

#[cfg(feature = "encode")]
pub mod encode;
