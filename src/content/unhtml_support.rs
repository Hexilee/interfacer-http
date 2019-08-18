#[allow(unused_imports)]
use super::encoding::disable_encoding_error;

#[cfg(feature = "encoding")]
use super::encoding::decode_data;

use super::error::FromContentError;
use crate::error::StringError;
use crate::mime::{self, Mime, UTF_8};
use crate::polyfill::FromContentHtml;
use unhtml::FromHtml;

impl<T: FromHtml> FromContentHtml for T {
    type Err = FromContentError;
    fn _from_content(data: Vec<u8>, content_type: &Mime) -> Result<Self, Self::Err> {
        if content_type.type_() == mime::TEXT && content_type.subtype() == mime::HTML {
            match content_type.get_param(mime::CHARSET) {
                None | Some(UTF_8) => Ok(T::from_html(&String::from_utf8(data)?)?),
                #[cfg(feature = "encoding")]
                Some(encoding) => Ok(T::from_html(&decode_data(&data, encoding.as_str())?)?),
                #[cfg(not(feature = "encoding"))]
                Some(encoding) => Err(disable_encoding_error(encoding.as_str()).into()),
            }
        } else {
            Err(StringError::new(format!("unsupported content type '{}'", content_type)).into())
        }
    }
}
