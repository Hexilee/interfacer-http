use super::User;
use interfacer_http_util::content_types::APPLICATION_JSON;

macro_rules! define_test {
    ($base_type:expr, $encoding:expr) => {
        let user = User {
            name: "hexi".to_owned(),
            age: 18,
        };
        let content_type = interfacer_http::ContentType::new($base_type, $encoding);
        let data = <User as interfacer_http::ToContent>::to_content(&user, &content_type)
            .expect(&format!("to '{}' fail", $base_type));
        let mirror = <User as interfacer_http::FromContent>::from_content(data, &content_type)
            .expect(&format!("from '{}' fail", $base_type));
        assert_eq!(user, mirror);
    };
}

#[cfg(any(feature = "serde-full", feature = "serde-json"))]
#[test]
fn json() {
    define_test!(APPLICATION_JSON, None);
}
