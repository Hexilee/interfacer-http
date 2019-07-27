use crate::content_types::ENCODING_UTF8;
use crate::fail::StringError;

#[allow(dead_code)]
pub fn encode_data(raw_data: String, encode: Option<&str>) -> Result<Vec<u8>, StringError> {
    match encode {
        None | Some(ENCODING_UTF8) => Ok(raw_data.into_bytes()),
        Some(encode) => implement::encode_data(raw_data.as_str(), encode),
    }
}

#[allow(dead_code)]
pub fn decode_data(raw_data: Vec<u8>, encode: Option<&str>) -> Result<String, StringError> {
    match encode {
        None | Some(ENCODING_UTF8) => match String::from_utf8(raw_data) {
            Ok(data) => Ok(data),
            Err(err) => Err(StringError::new(format!(
                "decode data error: encoding is not utf-8. cause by `{}`",
                err
            ))),
        },
        Some(encode) => implement::decode_data(&raw_data, encode),
    }
}

#[cfg(not(feature = "encode"))]
mod implement {
    use crate::fail::StringError;

    pub fn encode_data(_raw_data: &str, _encode: &str) -> Result<Vec<u8>, StringError> {
        Err(StringError::new(
            "encode feature is disable, please enable it",
        ))
    }

    pub fn decode_data(_raw_data: &[u8], _encode: &str) -> Result<String, StringError> {
        Err(StringError::new(
            "encode feature is disable, please enable it",
        ))
    }
}

#[cfg(feature = "encode")]
pub mod implement {
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
                .encode(&raw_data, encoding::EncoderTrap::Strict)
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
