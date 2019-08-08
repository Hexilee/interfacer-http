// TODO: concat content-types and encodings

// encoding
pub const CHARSET_UTF8: &str = "utf-8";
pub const CHARSET_GB2312: &str = "gb2312";
pub const CHARSET_GBK: &str = "gbk";

// content-type
pub const APPLICATION_JSON: &str = "application/json";
pub const APPLICATION_JAVA_SCRIPT: &str = "application/javascript";
pub const APPLICATION_XML: &str = "application/xml";
pub const TEXT_XML: &str = "text/xml";
pub const APPLICATION_FORM: &str = "application/x-www-form-urlencoded";
pub const APPLICATION_PROTOBUF: &str = "application/protobuf";
pub const APPLICATION_MSGPACK: &str = "application/msgpack";
pub const TEXT_HTML: &str = "text/html";
pub const TEXT_PLAIN: &str = "text/plain";
pub const MULTIPART_FORM: &str = "multipart/form-data";
pub const OCTET_STREAM: &str = "application/octet-stream";

macro_rules! const_join {
    ($base_type:expr, $encoding:expr) => {
        const_concat!($base_type, "; charset=", $encoding)
    };
}

pub const APPLICATION_JSON_CHARSET_UTF8: &str = const_join!(APPLICATION_JSON, CHARSET_UTF8);
