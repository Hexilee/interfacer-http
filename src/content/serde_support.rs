#![allow(unused_imports)]
use super::encoding::disable_encoding_error;
#[cfg(feature = "encoding")]
use super::encoding::{decode_data, encode_data};
use super::fail::{FromContentFail, ToContentFail};
use crate::content_type::ContentType;
use crate::content_types::*;
use crate::fail::StringError;
use crate::polyfill::{FromContentSerde, ToContentSerde};
use crate::url::form_urlencoded::Serializer as UrlEncodedSerializer;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
use std::io::Cursor;

impl<T: Serialize> ToContentSerde for T {
    type Err = ToContentFail;
    fn _to_content(&self, content_type: &ContentType) -> Result<Vec<u8>, Self::Err> {
        match content_type.base_type() {
            #[cfg(any(feature = "serde-full", feature = "serde-json"))]
            APPLICATION_JSON => {
                let data = serde_json::to_string(self)?;
                match content_type.encoding() {
                    None | Some(CHARSET_UTF8) => Ok(data.into_bytes()),
                    #[cfg(feature = "encoding")]
                    Some(encoding) => Ok(encode_data(data.as_str(), encoding)?),
                    #[cfg(not(feature = "encoding"))]
                    Some(encoding) => Err(disable_encoding_error(encoding).into()),
                }
            }

            #[cfg(any(feature = "serde-full", feature = "serde-xml"))]
            APPLICATION_XML | TEXT_XML => {
                let data = serde_xml_rs::to_string(self)?;
                match content_type.encoding() {
                    None | Some(CHARSET_UTF8) => Ok(data.into_bytes()),
                    #[cfg(feature = "encoding")]
                    Some(encoding) => Ok(encode_data(data.as_str(), encoding)?),
                    #[cfg(not(feature = "encoding"))]
                    Some(encoding) => Err(disable_encoding_error(encoding).into()),
                }
            }

            #[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
            APPLICATION_FORM => {
                match content_type.encoding() {
                    None | Some(CHARSET_UTF8) => Ok(serde_urlencoded::to_string(self)?.into_bytes()),
                    #[cfg(feature = "encoding")]
                    Some(encoding) => Ok(encode_into_form(self, |raw_str| {
                        match encode_data(raw_str, encoding) {
                            Ok(data) => Cow::Owned(data),
                            Err(_) => Cow::Borrowed(raw_str.as_bytes()), // Fixme: throw error when encoding fails
                        }
                    })?
                    .into_bytes()),
                    #[cfg(not(feature = "encoding"))]
                    Some(encoding) => Err(disable_encoding_error(encoding).into()),
                }
            }

            #[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
            APPLICATION_MSGPACK => Ok(rmp_serde::to_vec(self)?),

            unsupported => {
                Err(StringError::new(format!("unsupported content type '{}'", unsupported)).into())
            }
        }
    }
}

impl<T: DeserializeOwned> FromContentSerde for T {
    type Err = FromContentFail;
    fn _from_content(data: Vec<u8>, content_type: &ContentType) -> Result<Self, Self::Err> {
        match content_type.base_type() {
            #[cfg(any(feature = "serde-full", feature = "serde-json"))]
            APPLICATION_JSON => match content_type.encoding() {
                None | Some(CHARSET_UTF8) => Ok(serde_json::from_slice(&data)?),
                #[cfg(feature = "encoding")]
                Some(encoding) => Ok(serde_json::from_str(&decode_data(&data, encoding)?)?),
                #[cfg(not(feature = "encoding"))]
                Some(encoding) => Err(disable_encoding_error(encoding).into()),
            },

            #[cfg(any(feature = "serde-full", feature = "serde-xml"))]
            APPLICATION_XML | TEXT_XML => match content_type.encoding() {
                None | Some(CHARSET_UTF8) => Ok(serde_xml_rs::from_reader(Cursor::new(data))?),
                #[cfg(feature = "encoding")]
                Some(encoding) => Ok(serde_xml_rs::from_str(&decode_data(&data, encoding)?)?),
                #[cfg(not(feature = "encoding"))]
                Some(encoding) => Err(disable_encoding_error(encoding).into()),
            },

            #[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
            APPLICATION_FORM => Ok(serde_urlencoded::from_bytes(&data)?),

            #[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
            APPLICATION_MSGPACK => Ok(rmp_serde::from_slice(&data)?),

            unsupported => {
                Err(StringError::new(format!("unsupported content type '{}'", unsupported)).into())
            }
        }
    }
}

/// Serializes a value into a `application/x-wwww-url-encoded` `String` buffer in custom encoding.
///
/// ```ignore
/// use std::borrow::Cow;
///
/// let meal = &[
///     ("bread", "baguette"),
///     ("fat", "butter"),
/// ];
///
/// fn caesar_cipher_encode(raw: &str) -> Cow<[u8]> {
///     Cow::Owned(raw.as_bytes().iter().map(|ascii| (ascii - 94) % 26 + 97).collect())
/// }
///
/// assert_eq!(
///     encode_into_form(meal, caesar_cipher_encode),
///     Ok("euhdg=edjxhwwh&idw=exwwhu".to_owned()));
/// ```
#[cfg(all(
    feature = "encoding",
    any(feature = "serde-full", feature = "serde-urlencoded")
))]
fn encode_into_form(
    input: impl Serialize,
    encoding: impl Fn(&str) -> Cow<[u8]>,
) -> Result<String, serde_urlencoded::ser::Error> {
    let mut urlencoder = UrlEncodedSerializer::new("".to_owned());
    urlencoder.encoding_override(Some(&encoding));
    input.serialize(serde_urlencoded::Serializer::new(&mut urlencoder))?;
    Ok(urlencoder.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn test_encode_into_form() {
        let meal = &[("bread", "baguette"), ("fat", "butter")];
        fn caesar_cipher_encode(raw: &str) -> Cow<[u8]> {
            Cow::Owned(
                raw.as_bytes()
                    .iter()
                    .map(|ascii| (ascii - 94) % 26 + 97)
                    .collect(),
            )
        }
        assert_eq!(
            encode_into_form(meal, caesar_cipher_encode),
            Ok("euhdg=edjxhwwh&idw=exwwhu".to_owned())
        );
    }
}
