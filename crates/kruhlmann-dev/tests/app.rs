use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::{Router, body::to_bytes};
use kruhlmann_dev::TexContext;
use tower::ServiceExt;

const STATIC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../static");
const TEX_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tex");

fn app() -> Router {
    let tex_context = TexContext::load_from_directory(TEX_DIR).unwrap();
    kruhlmann_dev::build_app(STATIC_DIR, tex_context)
}

async fn send(app: Router, request: Request<Body>) -> axum::response::Response {
    app.oneshot(request).await.unwrap()
}

async fn body_string(response: axum::response::Response) -> String {
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn get_index_returns_rendered_page() {
    let response = send(
        app(),
        Request::builder().uri("/").body(Body::empty()).unwrap(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.starts_with("text/html"))
    );
    let body = body_string(response).await;
    assert!(body.contains("<!DOCTYPE"));
    assert!(body.contains("Andreas"));
}

#[tokio::test]
async fn post_to_index_is_method_not_allowed() {
    let response = send(
        app(),
        Request::builder()
            .method(axum::http::Method::POST)
            .uri("/")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn serves_existing_static_css_file() {
    let response = send(
        app(),
        Request::builder()
            .uri("/css/index.css")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.starts_with("text/css"))
    );
}

#[tokio::test]
async fn serves_existing_static_binary_file() {
    let response = send(
        app(),
        Request::builder()
            .uri("/img/avatar.png")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok()),
        Some("image/png")
    );
}

#[tokio::test]
async fn missing_static_file_returns_404() {
    let response = send(
        app(),
        Request::builder()
            .uri("/does-not-exist.html")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn unknown_api_like_path_does_not_hit_index_handler() {
    let response = send(
        app(),
        Request::builder()
            .uri("/about")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn path_traversal_outside_static_dir_is_rejected() {
    let response = send(
        app(),
        Request::builder()
            .uri("/../Cargo.toml")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    let status = response.status();
    let body = body_string(response).await;
    let ok_but_leaky = status == StatusCode::OK && body.contains("[package]");
    assert!(!ok_but_leaky);
}
