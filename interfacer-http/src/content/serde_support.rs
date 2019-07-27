#[allow(unused_imports)]
use super::encode::{decode_data, encode_data};
use super::fail::{FromContentFail, ToContentFail};
use crate::content_type::ContentType;
use crate::content_types::*;
use crate::fail::StringError;
//use crate::{FromContent, ToContent};
use crate::polyfill::{FromContentSerde, ToContentSerde};
use serde::{de::DeserializeOwned, Serialize};

impl<T: Serialize> ToContentSerde for T {
    type Err = ToContentFail;
    fn to_content(&self, content_type: &ContentType) -> Result<Vec<u8>, Self::Err> {
        match content_type.base_type() {
            #[cfg(any(feature = "serde-full", feature = "serde-json"))]
            APPLICATION_JSON => Ok(encode_data(
                serde_json::to_string(self)?,
                content_type.encoding(),
            )?),

            #[cfg(any(feature = "serde-full", feature = "serde-xml"))]
            APPLICATION_XML | TEXT_XML => Ok(encode_data(
                serde_xml_rs::to_string(self)?,
                content_type.encoding(),
            )?),

            #[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
            APPLICATION_FORM => Ok(encode_data(
                serde_urlencoded::to_string(self)?,
                content_type.encoding(),
            )?),

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
    fn from_content(data: Vec<u8>, content_type: &ContentType) -> Result<Self, Self::Err> {
        match content_type.base_type() {
            #[cfg(any(feature = "serde-full", feature = "serde-json"))]
            APPLICATION_JSON => Ok(serde_json::from_str(&decode_data(
                data,
                content_type.encoding(),
            )?)?),

            #[cfg(any(feature = "serde-full", feature = "serde-xml"))]
            APPLICATION_XML | TEXT_XML => Ok(serde_xml_rs::from_str(&decode_data(
                data,
                content_type.encoding(),
            )?)?),

            #[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
            APPLICATION_FORM => Ok(serde_urlencoded::from_str(&decode_data(
                data,
                content_type.encoding(),
            )?)?),

            #[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
            APPLICATION_MSGPACK => Ok(rmp_serde::from_slice(&data)?),

            unsupported => {
                Err(StringError::new(format!("unsupported content type '{}'", unsupported)).into())
            }
        }
    }
}
