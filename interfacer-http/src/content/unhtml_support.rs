use super::encode::decode_data;
use super::fail::FromContentFail;
use crate::content_type::ContentType;
use crate::content_types::*;
use crate::fail::StringError;
//use crate::FromContent;
use crate::polyfill::FromContentHtml;
use unhtml::FromHtml;

impl<T: FromHtml> FromContentHtml for T {
    type Err = FromContentFail;
    fn from_content(data: Vec<u8>, content_type: &ContentType) -> Result<Self, Self::Err> {
        match content_type.base_type() {
            TEXT_HTML => Ok(Self::from_html(&decode_data(
                data,
                content_type.encoding(),
            )?)?),

            unsupported => {
                Err(StringError::new(format!("unsupported content type '{}'", unsupported)).into())
            }
        }
    }
}
