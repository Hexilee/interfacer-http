use interfacer_http::polyfill::*;
use interfacer_http::ContentType;
use unhtml_derive::FromHtml;

#[derive(FromHtml)]
#[html(selector = "a")]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

#[test]
fn normal() {
    let link = Link::from_content(
        br#"<a href="https://github.com">Github</a>"#[..].to_vec(),
        &ContentType::new("text/html", None),
    )
    .expect("from html fail");
    assert_eq!("https://github.com", &link.href);
    assert_eq!("Github", &link.value);
}
