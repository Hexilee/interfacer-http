use crate::{FromContentError, ToContentError};

use lib_encoding::label::encoding_from_whatwg_label;

#[allow(dead_code)]
pub fn encode_data(raw_data: &str, encoding: &str) -> Result<Vec<u8>, ToContentError> {
    match encoding_from_whatwg_label(encoding) {
        Some(encoder) => encoder
            .encode(&raw_data, lib_encoding::EncoderTrap::Strict)
            .map_err(|err| ToContentError::EncodeError(format!("{}", err))),
        None => Err(ToContentError::UnsupportedEncoding(encoding.into())),
    }
}

#[allow(dead_code)]
pub fn decode_data(raw_data: &[u8], encoding: &str) -> Result<String, FromContentError> {
    match encoding_from_whatwg_label(encoding) {
        Some(encoder) => encoder
            .decode(raw_data, lib_encoding::DecoderTrap::Strict)
            .map_err(|err| FromContentError::DecodeError(format!("{}", err))),
        None => Err(FromContentError::UnsupportedEncoding(encoding.into())),
    }
}
