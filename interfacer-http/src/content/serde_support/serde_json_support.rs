use super::super::encode::{decode_data, encode_data};
use super::{FromContentFail, ToContentFail};
use crate::content_types::ENCODING_UTF8;
use crate::{FromContent, ToContent};
use serde::{de::DeserializeOwned, Serialize};

impl<T: Serialize> ToContent<"application/json"> for T {
    type Err = ToContentFail;
    fn to_content(&self, encode: Option<&str>) -> Result<Vec<u8>, Self::Err> {
        match encode {
            None | Some(ENCODING_UTF8) => Ok(serde_json::to_vec(self)?),
            Some(encode) => Ok(encode_data(&serde_json::to_string(self)?, encode)?),
        }
    }
}

impl<T: DeserializeOwned> FromContent<"application/json"> for T {
    type Err = FromContentFail;
    fn from_content(data: &[u8], encode: Option<&str>) -> Result<Self, Self::Err> {
        match encode {
            None | Some(ENCODING_UTF8) => Ok(serde_json::from_slice(data)?),
            Some(encode) => Ok(serde_json::from_str(decode_data(data, encode)?.as_str())?),
        }
    }
}
