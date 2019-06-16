pub const CHARSET_UTF8: &str = "charset=UTF-8";
pub const APPLICATION_JSON: &str = "application/json";
pub const APPLICATION_JSON_CHARSET_UTF8: &str = const_concat!(APPLICATION_JSON, "; ", CHARSET_UTF8);
pub const APPLICATION_JAVA_SCRIPT: &str = "application/javascript";
pub const APPLICATION_JAVA_SCRIPT_CHARSET_UTF8: &str =
    const_concat!(APPLICATION_JAVA_SCRIPT, "; ", CHARSET_UTF8);
pub const APPLICATION_XML: &str = "application/xml";
pub const APPLICATION_XML_CHARSET_UTF8: &str = const_concat!(APPLICATION_XML, "; ", CHARSET_UTF8);
pub const TEXT_XML: &str = "text/xml";
pub const TEXT_XML_CHARSET_UTF8: &str = const_concat!(TEXT_XML, "; ", CHARSET_UTF8);
pub const APPLICATION_FORM: &str = "application/x-www-form-urlencoded";
pub const APPLICATION_PROTOBUF: &str = "application/protobuf";
pub const APPLICATION_MSGPACK: &str = "application/msgpack";
pub const TEXT_HTML: &str = "text/html";
pub const TEXT_HTML_CHARSET_UTF8: &str = const_concat!(TEXT_HTML, "; ", CHARSET_UTF8);
pub const TEXT_PLAIN: &str = "text/plain";
pub const TEXT_PLAIN_CHARSET_UTF8: &str = const_concat!(TEXT_PLAIN, "; ", CHARSET_UTF8);
pub const MULTIPART_FORM: &str = "multipart/form-data";
pub const OCTET_STREAM: &str = "application/octet-stream";
