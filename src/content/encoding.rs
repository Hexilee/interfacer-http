use crate::content_type::ContentType;
use crate::fail::StringError;

#[allow(dead_code)]
pub fn check_encoding(content_type: &ContentType) -> Result<(), StringError> {
    match content_type.encoding() {
        #[cfg(not(feature = "encoding"))]
        Some(encoding) if encoding != crate::content_types::CHARSET_UTF8 => {
            Err(StringError::new(format!(
                "unsupported encoding: {}; please enable feature `encoding`",
                encoding
            )))
        }
        _ => Ok(()),
    }
}

#[cfg(feature = "encoding")]
use lib_encoding::label::encoding_from_whatwg_label;

#[cfg(feature = "encoding")]
#[allow(dead_code)]
pub fn encode_data(raw_data: &str, encode: &str) -> Result<Vec<u8>, StringError> {
    match encoding_from_whatwg_label(encode) {
        Some(encoder) => encoder
            .encode(&raw_data, lib_encoding::EncoderTrap::Strict)
            .map_err(|err| StringError::new(format!("{}", err))),
        None => Err(StringError::new("unsupported encoding")),
    }
}

#[cfg(feature = "encoding")]
#[allow(dead_code)]
pub fn decode_data(raw_data: &[u8], encode: &str) -> Result<String, StringError> {
    match encoding_from_whatwg_label(encode) {
        Some(encoder) => encoder
            .decode(raw_data, lib_encoding::DecoderTrap::Strict)
            .map_err(|err| StringError::new(format!("{}", err))),
        None => Err(StringError::new("unsupported encoding")),
    }
}