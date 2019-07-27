#[cfg(all(
    feature = "derive",
    any(feature = "serde-base", feature = "serde-full")
))]
mod serde_support;

#[cfg(all(feature = "derive", feature = "unhtml-html"))]
mod unhtml_support;

#[macro_export]
macro_rules! define_mirror_test {
    ($typ:ident, $object:expr, $base_type:expr, $encoding:expr) => {
        let content_type = interfacer_http::ContentType::new($base_type, $encoding, None);
        let data = $object
            .to_content(&content_type)
            .expect(&format!("to '{}' fail", $base_type));
        let mirror =
            $typ::from_content(data, &content_type).expect(&format!("from '{}' fail", $base_type));
        assert_eq!($object, mirror);
    };
}
