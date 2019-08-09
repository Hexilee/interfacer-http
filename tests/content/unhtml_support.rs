use interfacer_http::derive::FromContent;
use interfacer_http::{ContentType, IntoStruct};
use unhtml_derive::FromHtml;

#[derive(FromHtml, FromContent)]
#[html(selector = "a")]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

#[test]
fn normal() {
    let link: Link = br#"<a href="https://github.com">Github</a>"#[..]
        .to_vec()
        .into_struct(&ContentType::new("text/html", None, None))
        .expect("from html fail");
    assert_eq!("https://github.com", &link.href);
    assert_eq!("Github", &link.value);
}
