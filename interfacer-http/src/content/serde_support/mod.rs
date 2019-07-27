use super::encode::{decode_data, encode_data};
use super::fail::{FromContentFail, ToContentFail};
use crate::content_type::ContentType;
use crate::content_types::*;
use crate::fail::StringError;
use crate::{FromContent, ToContent};
use serde::{de::DeserializeOwned, Serialize};

impl<T: Serialize> ToContent for T {
    type Err = ToContentFail;
    fn to_content(&self, content_type: ContentType) -> Result<Vec<u8>, Self::Err> {
        match content_type.base_type() {
            #[cfg(any(feature = "serde-full", feature = "serde-json"))]
            APPLICATION_JSON => Ok(encode_data(
                serde_json::to_string(self)?,
                content_type.encoding(),
            )?),
            unsupported => {
                Err(StringError::new(format!("unsupported content type '{}'", unsupported)).into())
            }
        }
    }
}

//impl<T: DeserializeOwned> FromContent for T {
//    type Err = FromContentFail;
//    fn from_content(data: &[u8], _encode: Option<&str>) -> Result<Self, Self::Err> {
//        Ok(rmp_serde::from_slice(data)?)
//    }
//}

//macro_rules! import {
//    () => {
//        use super::super::encode::{decode_data, encode_data};
//        use super::{FromContentFail, ToContentFail};
//        use crate::content_types::ENCODING_UTF8;
//        use crate::{FromContent, ToContent};
//        use serde::{de::DeserializeOwned, Serialize};
//    };
//}
//
//macro_rules! define_support {
//    ($content_type:expr, $to_vec:path, $to_string:path, $from_slice:path, $from_str:path) => {
//        impl<T: Serialize> ToContent<$content_type> for T {
//            type Err = ToContentFail;
//            fn to_content(&self, encode: Option<&str>) -> Result<Vec<u8>, Self::Err> {
//                match encode {
//                    None | Some(ENCODING_UTF8) => Ok($to_vec(self)?),
//                    Some(encode) => Ok(encode_data($to_string(self)?.as_str(), encode)?),
//                }
//            }
//        }
//
//        impl<T: DeserializeOwned> FromContent<$content_type> for T {
//            type Err = FromContentFail;
//            fn from_content(data: &[u8], encode: Option<&str>) -> Result<Self, Self::Err> {
//                match encode {
//                    None | Some(ENCODING_UTF8) => Ok($from_slice(data)?),
//                    Some(encode) => Ok($from_str(decode_data(data, encode)?.as_str())?),
//                }
//            }
//        }
//    };
//}
//
//#[cfg(any(feature = "serde-full", feature = "serde-json"))]
//mod serde_json_support {
//    use serde_json::{from_slice, from_str, to_string, to_vec};
//    import!();
//    define_support!("application/json", to_vec, to_string, from_slice, from_str);
//}
//
//#[cfg(any(feature = "serde-full", feature = "serde-urlencoded"))]
//mod serde_urlencoded_support {
//    use serde_urlencoded::{from_bytes, from_str, to_string};
//    import!();
//    fn to_vec(object: impl Serialize) -> Result<Vec<u8>, ToContentFail> {
//        Ok(to_string(&object)?.into_bytes())
//    }
//    define_support!(
//        "application/x-www-form-urlencoded",
//        to_vec,
//        to_string,
//        from_bytes,
//        from_str
//    );
//}
//
//#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
//mod serde_xml_support {
//    use serde_xml_rs::{from_reader, from_str, to_string, to_writer};
//    import!();
//
//    fn to_vec(object: impl Serialize) -> Result<Vec<u8>, ToContentFail> {
//        let mut data = Vec::new();
//        to_writer(&mut data, &object)?;
//        Ok(data)
//    }
//
//    define_support!("application/xml", to_vec, to_string, from_reader, from_str);
//    define_support!("text/xml", to_vec, to_string, from_reader, from_str);
//}
//
//#[cfg(any(feature = "serde-full", feature = "serde-msgpack"))]
//mod serde_msgpack_support {
//    use super::{FromContentFail, ToContentFail};
//    use crate::{FromContent, ToContent};
//    use serde::{de::DeserializeOwned, Serialize};
//
//    impl<T: Serialize> ToContent<"application/msgpack"> for T {
//        type Err = ToContentFail;
//        fn to_content(&self, _encode: Option<&str>) -> Result<Vec<u8>, Self::Err> {
//            Ok(rmp_serde::to_vec(self)?)
//        }
//    }
//
//    impl<T: DeserializeOwned> FromContent<"application/msgpack"> for T {
//        type Err = FromContentFail;
//        fn from_content(data: &[u8], _encode: Option<&str>) -> Result<Self, Self::Err> {
//            Ok(rmp_serde::from_slice(data)?)
//        }
//    }
//}
