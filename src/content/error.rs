use crate::mime::{Mime, UTF_8};
use derive_more::{Display, From};

/// Error type for `ToContent`.
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

/// Error type for `FromContent`.
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

impl From<std::string::FromUtf8Error> for FromContentError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        let msg = err.to_string();
        (err.into_bytes(), UTF_8.to_string(), msg).into()
    }
}
