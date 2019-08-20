// TODO: test this mod

#[cfg(feature = "encoding")]
use super::encoding::{decode_data, encode_data};
use super::error::{FromContentError, ToContentError};
use crate::mime::{
    Mime, Name, APPLICATION, APPLICATION_JSON, APPLICATION_MSGPACK,
    APPLICATION_WWW_FORM_URLENCODED, CHARSET, JSON, MSGPACK, TEXT, TEXT_XML, UTF_8,
    WWW_FORM_URLENCODED, XML,
};
use crate::polyfill::{FromContentSerde, ToContentSerde};
use crate::url::form_urlencoded::Serializer as UrlEncodedSerializer;
use crate::MimeExt;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;

impl<T: Serialize> ToContentSerde for T {
    type Err = ToContentError;
    fn _to_content(&self, content_type: &Mime) -> Result<Vec<u8>, Self::Err> {
        match (content_type.type_(), content_type.subtype()) {
            #[cfg(any(feature = "serde-full", feature = "serde-json"))]
            (APPLICATION, JSON) => to_json(self, content_type.get_param(CHARSET)),

            #[cfg(any(feature = "serde-full", feature = "serde-xml"))]
            (APPLICATION, XML) | (TEXT, XML) => to_xml(self, content_type.get_param(CHARSET)),

            #[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
            (APPLICATION, WWW_FORM_URLENCODED) => to_form(self, content_type.get_param(CHARSET)),

            #[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
            (APPLICATION, MSGPACK) => Ok(rmp_serde::to_vec(self)?),

            _ => Err(content_type.pure_type().into()),
        }
    }
}

impl<T: DeserializeOwned> FromContentSerde for T {
    type Err = FromContentError;
    fn _from_content(data: Vec<u8>, content_type: &Mime) -> Result<Self, Self::Err> {
        match (content_type.type_(), content_type.subtype()) {
            #[cfg(any(feature = "serde-full", feature = "serde-json"))]
            (APPLICATION, JSON) => from_json(data, content_type.get_param(CHARSET)),

            #[cfg(any(feature = "serde-full", feature = "serde-xml"))]
            (APPLICATION, XML) | (TEXT, XML) => from_xml(data, content_type.get_param(CHARSET)),

            #[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
            (APPLICATION, WWW_FORM_URLENCODED) => Ok(serde_urlencoded::from_bytes(&data)
                .map_err(|err| (data, content_type.pure_type(), err.to_string()))?),

            #[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
            (APPLICATION, MSGPACK) => Ok(rmp_serde::from_slice(&data)
                .map_err(|err| (data, content_type.pure_type(), err.to_string()))?),

            _ => Err(content_type.pure_type().into()),
        }
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
fn to_json(src: &impl Serialize, charset: Option<Name>) -> Result<Vec<u8>, ToContentError> {
    let data = serde_json::to_string(src)?;
    match charset {
        None | Some(UTF_8) => Ok(data.into_bytes()),
        #[cfg(feature = "encoding")]
        Some(encoding) => Ok(encode_data(data.as_str(), encoding.as_str())?),
        #[cfg(not(feature = "encoding"))]
        Some(encoding) => Err(encoding.to_string().into()),
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
fn from_json<T: DeserializeOwned>(
    data: Vec<u8>,
    charset: Option<Name>,
) -> Result<T, FromContentError> {
    match charset {
        None | Some(UTF_8) => Ok(serde_json::from_slice(&data)
            .map_err(|err| (data, APPLICATION_JSON, err.to_string()))?),
        #[cfg(feature = "encoding")]
        Some(encoding) => {
            let data = decode_data(&data, encoding.as_str())?;
            Ok(serde_json::from_str(&data)
                .map_err(|err| (data.into_bytes(), APPLICATION_JSON, err.to_string()))?)
        }
        #[cfg(not(feature = "encoding"))]
        Some(encoding) => Err(encoding.to_string().into()),
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
fn to_xml(src: &impl Serialize, charset: Option<Name>) -> Result<Vec<u8>, ToContentError> {
    let data = serde_xml_rs::to_string(src)?;
    match charset {
        None | Some(UTF_8) => Ok(data.into_bytes()),
        #[cfg(feature = "encoding")]
        Some(encoding) => Ok(encode_data(data.as_str(), encoding.as_str())?),
        #[cfg(not(feature = "encoding"))]
        Some(encoding) => Err(encoding.to_string().into()),
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
fn from_xml<T: DeserializeOwned>(
    data: Vec<u8>,
    charset: Option<Name>,
) -> Result<T, FromContentError> {
    match charset {
        None | Some(UTF_8) => {
            Ok(serde_urlencoded::from_bytes(&data)
                .map_err(|err| (data, TEXT_XML, err.to_string()))?)
        }
        #[cfg(feature = "encoding")]
        Some(encoding) => {
            let data = decode_data(&data, encoding.as_str())?;
            Ok(serde_json::from_str(&data)
                .map_err(|err| (data.into_bytes(), TEXT_XML, err.to_string()))?)
        }
        #[cfg(not(feature = "encoding"))]
        Some(encoding) => Err(encoding.to_string().into()),
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
fn to_form(src: &impl Serialize, charset: Option<Name>) -> Result<Vec<u8>, ToContentError> {
    match charset {
        None | Some(UTF_8) => Ok(serde_urlencoded::to_string(src)?.into_bytes()),
        #[cfg(feature = "encoding")]
        Some(encoding) => Ok(encode_into_form(src, &|raw_str| {
            match encode_data(raw_str, encoding.as_str()) {
                Ok(data) => Cow::Owned(data),
                Err(_) => Cow::Borrowed(raw_str.as_bytes()), // Fixme: throw error when encoding fails
            }
        })?
        .into_bytes()),
        #[cfg(not(feature = "encoding"))]
        Some(encoding) => Err(encoding.to_string().into()),
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
impl From<serde_json::Error> for ToContentError {
    fn from(err: serde_json::Error) -> Self {
        (APPLICATION_JSON, err.to_string()).into()
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
impl From<serde_xml_rs::Error> for ToContentError {
    fn from(err: serde_xml_rs::Error) -> Self {
        (TEXT_XML, err.to_string()).into()
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
impl From<serde_urlencoded::ser::Error> for ToContentError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        (APPLICATION_WWW_FORM_URLENCODED, err.to_string()).into()
    }
}

#[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
impl From<rmp_serde::encode::Error> for ToContentError {
    fn from(err: rmp_serde::encode::Error) -> Self {
        (APPLICATION_MSGPACK, err.to_string()).into()
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
    input: &impl Serialize,
    encoding: &dyn Fn(&str) -> Cow<[u8]>,
) -> Result<String, serde_urlencoded::ser::Error> {
    let mut urlencoder = UrlEncodedSerializer::new("".to_owned());
    urlencoder.encoding_override(Some(encoding));
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
