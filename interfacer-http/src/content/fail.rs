use crate::define_from as request_fail_define_from;
use crate::fail::StringError;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum ToContentFail {
    #[fail(display = "to content fail: {}", err)]
    Inner { err: Box<dyn Fail> },
}

#[derive(Fail, Debug)]
pub enum FromContentFail {
    #[fail(display = "from content fail: {}", err)]
    Inner { err: Box<dyn Fail> },
}

// from: Fail
macro_rules! define_from {
    ($from:ty) => {
        define_from!($from, FromContentFail);
        define_from!($from, ToContentFail);
    };

    ($from:ty, $to:ty) => {
        impl From<$from> for $to {
            fn from(err: $from) -> Self {
                Self::Inner { err: Box::new(err) }
            }
        }
    };
}

// from: Display
macro_rules! define_from_by_str {
    ($from:ty) => {
        define_from_by_str!($from, FromContentFail);
        define_from_by_str!($from, ToContentFail);
    };

    ($from:ty, $to:ty) => {
        impl From<$from> for $to {
            fn from(err: $from) -> Self {
                StringError::new(format!("{}", err)).into()
            }
        }
    };
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
define_from!(serde_json::Error);

#[cfg(any(feature = "serde-full", feature = "serde-xml"))]
define_from_by_str!(serde_xml_rs::Error);

define_from!(crate::fail::StringError);

request_fail_define_from!(FromContentFail);
request_fail_define_from!(ToContentFail);
