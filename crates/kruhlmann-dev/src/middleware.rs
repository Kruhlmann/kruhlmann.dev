use axum::{
    extract::Request,
    http::header::{CONTENT_TYPE, USER_AGENT},
    middleware::Next,
    response::Response,
};

pub async fn log_visit(request: Request, next: Next) -> Response {
    let path = request.uri().path().to_owned();
    let user_agent = request
        .headers()
        .get(USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown")
        .to_owned();
    let response = next.run(request).await;
    let is_html = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.starts_with("text/html"));
    if is_html {
        tracing::debug!(?path, ?user_agent, "visit")
    }
    response
}
