use super::super::encode::find_encoder;
use super::{FromContentFail, ToContentFail};
use crate::content_types::ENCODING_UTF8;
use crate::fail::StringError;
use crate::{FromContent, ToContent};
use serde::{de::DeserializeOwned, Serialize};

impl<T: Serialize> ToContent<"application/json"> for T {
    type Err = ToContentFail;
    fn to_content(&self, encode: Option<&str>) -> Result<Vec<u8>, Self::Err> {
        match encode {
            None | Some(ENCODING_UTF8) => Ok(serde_json::to_vec(self)?),
            Some(encode) => Ok(encode_data(&serde_json::to_string(self)?, encode)?),
        }
    }
}

impl<T: DeserializeOwned> FromContent<"application/json"> for T {
    type Err = FromContentFail;
    fn from_content(data: &[u8], encode: Option<&str>) -> Result<Self, Self::Err> {
        match encode {
            None | Some(ENCODING_UTF8) => Ok(serde_json::from_slice(data)?),
            Some(encode) => Ok(serde_json::from_str(decode_data(data, encode)?.as_str())?),
        }
    }
}

#[cfg(not(feature = "encode"))]
fn encode_data(raw_data: &str, _encode: &str) -> Result<Vec<u8>, StringError> {
    panic!("encode feature is disable, please enable it");
}

#[cfg(feature = "encode")]
fn encode_data(raw_data: &str, encode: &str) -> Result<Vec<u8>, StringError> {
    match find_encoder(encode) {
        Some(encoder) => encoder
            .encode(raw_data, encoding::EncoderTrap::Strict)
            .map_err(|err| StringError::new(format!("{}", err))),
        None => Err(StringError::new("unsupported encoding")),
    }
}

#[cfg(not(feature = "encode"))]
fn decode_data(raw_data: &[u8], _encode: &str) -> Result<String, StringError> {
    panic!("encode feature is disable, please enable it");
}

#[cfg(feature = "encode")]
fn decode_data(raw_data: &[u8], encode: &str) -> Result<String, StringError> {
    match find_encoder(encode) {
        Some(encoder) => encoder
            .decode(raw_data, encoding::DecoderTrap::Strict)
            .map_err(|err| StringError::new(format!("{}", err))),
        None => Err(StringError::new("unsupported encoding")),
    }
}
