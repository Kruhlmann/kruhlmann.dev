use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub struct HtmlTemplate<T>(pub T);

pub fn template_response(rendered: Result<String, askama::Error>) -> Response {
    match rendered {
        Ok(html) => Html(html).into_response(),
        Err(error) => {
            tracing::error!(?error, "failed to render template");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> Response {
        template_response(self.0.render())
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexHtmlTemplate {}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn index_template_renders_expected_content() {
        let html = IndexHtmlTemplate {}.render().unwrap();
        assert!(html.contains("<!DOCTYPE"));
        assert!(html.contains("Andreas"));
        assert!(html.contains("/css/index.css"));
        assert!(!html.contains("{%"));
        assert!(!html.contains("{{"));
    }

    #[tokio::test]
    async fn html_template_success_returns_200_with_html_body() {
        let response = HtmlTemplate(IndexHtmlTemplate {}).into_response();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|v| v.starts_with("text/html"))
        );
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("Andreas"));
    }

    #[tokio::test]
    async fn template_response_error_returns_500() {
        let error = askama::Error::custom(std::io::Error::other("render failure"));
        let response = template_response(Err(error));
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn template_response_success_returns_200() {
        let response = template_response(Ok("<h1>hi</h1>".to_string()));
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(std::str::from_utf8(&body).unwrap(), "<h1>hi</h1>");
    }
}
