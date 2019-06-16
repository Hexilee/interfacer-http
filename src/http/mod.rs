pub trait FromContent<const CONTENT_TYPE: &'static str> {
    fn from_content(data: &[u8]) -> Self;
}

pub trait ToContent<const CONTENT_TYPE: &'static str> {
    fn to_content(&self) -> &[u8];
}

pub mod content_type;
