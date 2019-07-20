use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct Expect {
    pub status: i32,
    pub content_type: Option<String>,
}