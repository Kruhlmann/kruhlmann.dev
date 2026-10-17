use crate::{HtmlTemplate, IndexHtmlTemplate};

pub async fn index() -> HtmlTemplate<IndexHtmlTemplate> {
    HtmlTemplate(IndexHtmlTemplate {})
}