#[allow(unused_imports)]
use crate::mime::{
    Mime, APPLICATION_JSON, APPLICATION_MSGPACK, APPLICATION_WWW_FORM_URLENCODED, TEXT_HTML,
    TEXT_XML, UTF_8,
};
use derive_more::{Display, From};

#[derive(Display, Debug, From)]
pub enum ToContentError {
    #[display(fmt = "unsupported content type: {}", _0)]
    UnsupportedContentType(Mime),

    #[display(fmt = "unsupported encoding: {}", _0)]
    UnsupportedEncoding(String),

    #[display(fmt = "serialize error: {}", msg)]
    SerializeError { content_type: Mime, msg: String },

    #[display(fmt = "encode error: {}", msg)]
    EncodeError {
        src: String,
        encoding: String,
        msg: String,
    },
}

#[derive(Display, Debug, From)]
pub enum FromContentError {
    #[display(fmt = "unsupported content type: {}", _0)]
    UnsupportedContentType(Mime),

    #[display(fmt = "unsupported encoding: {}", _0)]
    UnsupportedEncoding(String),

    #[display(fmt = "deserialize error: {}", msg)]
    DeserializerError {
        data: Vec<u8>,
        content_type: Mime,
        msg: String,
    },

    #[display(fmt = "decode error: {}", msg)]
    DecodeError {
        src: Vec<u8>,
        encoding: String,
        msg: String,
    },
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

impl From<std::string::FromUtf8Error> for FromContentError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        let msg = err.to_string();
        (err.into_bytes(), UTF_8.to_string(), msg).into()
    }
}
