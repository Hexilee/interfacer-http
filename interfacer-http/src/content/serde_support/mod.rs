use super::fail::{FromContentFail, ToContentFail};

macro_rules! import {
    () => {
        use super::super::encode::{decode_data, encode_data};
        use super::{FromContentFail, ToContentFail};
        use crate::content_types::ENCODING_UTF8;
        use crate::{FromContent, ToContent};
        use serde::{de::DeserializeOwned, Serialize};
    };
}

macro_rules! define_support {
    ($content_type:expr, $to_vec:path, $to_string:path, $from_slice:path, $from_str:path) => {
        impl<T: Serialize> ToContent<"application/json"> for T {
            type Err = ToContentFail;
            fn to_content(&self, encode: Option<&str>) -> Result<Vec<u8>, Self::Err> {
                match encode {
                    None | Some(ENCODING_UTF8) => Ok($to_vec(self)?),
                    Some(encode) => Ok(encode_data($to_string(self)?.as_str(), encode)?),
                }
            }
        }

        impl<T: DeserializeOwned> FromContent<"application/json"> for T {
            type Err = FromContentFail;
            fn from_content(data: &[u8], encode: Option<&str>) -> Result<Self, Self::Err> {
                match encode {
                    None | Some(ENCODING_UTF8) => Ok($from_slice(data)?),
                    Some(encode) => Ok($from_str(decode_data(data, encode)?.as_str())?),
                }
            }
        }
    };
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
mod serde_json_support {
    use serde_json::{from_slice, from_str, to_string, to_vec};
    import!();
    define_support!("application/json", to_vec, to_string, from_slice, from_str);
}
