use interfacer_http::derive::FromContent;
use interfacer_http::{ContentInto, ContentType};
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
        .content_into(&ContentType::new("text/html", None, None))
        .expect("from html fail");
    assert_eq!("https://github.com", &link.href);
    assert_eq!("Github", &link.value);
}
