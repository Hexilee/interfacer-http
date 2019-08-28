#[cfg(feature = "encoding")]
use super::encoding::decode_data;

use super::error::FromContentError;
use crate::mime::{self, Mime, HTML, TEXT, UTF_8};
use crate::polyfill::FromContentHtml;
use crate::MimeExt;
use unhtml::FromHtml;

impl<T> FromContentHtml for T
where
    T: FromHtml,
{
    fn _from_content(data: Vec<u8>, content_type: &Mime) -> Result<Self, FromContentError> {
        match (content_type.type_(), content_type.subtype()) {
            (TEXT, HTML) => match content_type.get_param(mime::CHARSET) {
                None | Some(UTF_8) => {
                    let str_data = String::from_utf8(data)?;
                    Ok(T::from_html(&str_data).map_err(|err| {
                        (
                            str_data.into_bytes(),
                            content_type.pure_type(),
                            err.to_string(),
                        )
                    })?)
                }
                #[cfg(feature = "encoding")]
                Some(encoding) => {
                    let decoded = decode_data(&data, encoding.as_str())?;
                    Ok(T::from_html(&decoded).map_err(|err| {
                        (
                            decoded.into_bytes(),
                            content_type.pure_type(),
                            err.to_string(),
                        )
                    })?)
                }
                #[cfg(not(feature = "encoding"))]
                Some(encoding) => Err(encoding.to_string().into()),
            },
            _ => Err(content_type.clone().into()),
        }
    }
}
