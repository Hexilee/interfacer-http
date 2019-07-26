use crate::define_from as request_fail_define_from;
use failure::Fail;

#[derive(Fail, Debug)]
pub enum IntoContentFail {
    #[fail(display = "into content fail: {}", err)]
    Inner { err: Box<dyn Fail> },
}

#[derive(Fail, Debug)]
pub enum FromContentFail {
    #[fail(display = "from content fail: {}", err)]
    Inner { err: Box<dyn Fail> },
}

macro_rules! define_from {
    ($from:ty) => {
        define_from!($from, FromContentFail);
        define_from!($from, IntoContentFail);
    };

    ($from:ty, $to:ty) => {
        impl From<$from> for $to {
            fn from(err: $from) -> Self {
                Self::Inner { err: Box::new(err) }
            }
        }
    };
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
define_from!(serde_json::Error);

#[cfg(feature = "encode")]
define_from!(std::borrow::Cow<'static, str>);

request_fail_define_from!(IntoContentFail);
