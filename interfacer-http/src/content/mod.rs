use crate::content_type::ContentType;
use crate::StdResult;

// TODO: use T: AsyncRead as type of data
// TODO: declare content_type as generics when const generics is stable
pub trait FromContent: Sized {
    type Err;
    fn from_content(data: &[u8], content_type: ContentType) -> StdResult<Self, Self::Err>;
}

// TODO: use T: AsyncRead as type of ret
// TODO: declare content_type as generics when const generics is stable
pub trait ToContent {
    type Err;
    fn to_content(&self, content_type: ContentType) -> StdResult<Vec<u8>, Self::Err>;
}

#[cfg(any(feature = "serde-base", feature = "serde-full"))]
mod serde_support;

pub mod fail;

#[cfg(not(feature = "encode"))]
pub mod encode {
    use crate::fail::StringError;

    pub fn encode_data(_raw_data: &str, _encode: &str) -> Result<Vec<u8>, StringError> {
        panic!("encode feature is disable, please enable it");
    }
    pub fn decode_data(_raw_data: &[u8], _encode: &str) -> Result<String, StringError> {
        panic!("encode feature is disable, please enable it");
    }
}

#[cfg(feature = "encode")]
pub mod encode {
    use crate::fail::StringError;
    use encoding::all::*;
    use encoding::EncodingRef;
    use std::collections::HashMap;

    thread_local! {
        static ENCODER_MAP: HashMap<&'static str, EncodingRef> = encodings()
            .into_iter()
            .map(|encoder| {
                if let Some(name) = encoder.whatwg_name() {
                    (name, *encoder)
                } else {
                    (encoder.name(), *encoder)
                }
            })
            .collect();
    }

    fn find_encoder(encoding: &str) -> Option<EncodingRef> {
        ENCODER_MAP.with(|map| map.get(encoding).map(|encoder| *encoder))
    }

    pub fn encode_data(raw_data: &str, encode: &str) -> Result<Vec<u8>, StringError> {
        match find_encoder(encode) {
            Some(encoder) => encoder
                .encode(raw_data, encoding::EncoderTrap::Strict)
                .map_err(|err| StringError::new(format!("{}", err))),
            None => Err(StringError::new("unsupported encoding")),
        }
    }

    pub fn decode_data(raw_data: &[u8], encode: &str) -> Result<String, StringError> {
        match find_encoder(encode) {
            Some(encoder) => encoder
                .decode(raw_data, encoding::DecoderTrap::Strict)
                .map_err(|err| StringError::new(format!("{}", err))),
            None => Err(StringError::new("unsupported encoding")),
        }
    }
}
