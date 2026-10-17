use std::path::Path;

use axum::Router;
use tokio::net::ToSocketAddrs;
use tower_http::services::ServeDir;

pub fn build_app<P: AsRef<Path>>(static_dir: P) -> Router {
    Router::new()
        .route("/", axum::routing::get(crate::router::index))
        .fallback_service(ServeDir::new(static_dir))
}

pub async fn bootstrap<A: ToSocketAddrs, P: AsRef<Path>>(
    address: A,
    static_dir: P,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_app(static_dir);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
