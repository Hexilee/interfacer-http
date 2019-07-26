use crate::StdResult;

// TODO: use T: AsyncRead as type of data
pub trait FromContent<const CONTENT_TYPE: &'static str>: Sized {
    type Err;
    fn from_content(data: &[u8], charset: Option<&str>) -> StdResult<Self, Self::Err>;
}

// TODO: use T: AsyncRead as type of ret
pub trait IntoContent<const CONTENT_TYPE: &'static str>: Sized {
    type Err;
    fn into_content(self, charset: Option<&str>) -> StdResult<Vec<u8>, Self::Err>;
}
