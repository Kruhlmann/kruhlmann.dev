use crate::{HtmlTemplate, IndexHtmlTemplate};

pub async fn index() -> HtmlTemplate<IndexHtmlTemplate> {
    HtmlTemplate(IndexHtmlTemplate {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn index_handler_returns_ok_html_response() {
        let response = index().await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|v| v.starts_with("text/html"))
        );
    }
}
