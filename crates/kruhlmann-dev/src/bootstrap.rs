use std::path::Path;

use axum::Router;
use tokio::net::ToSocketAddrs;
use tower_http::services::ServeDir;

use crate::{AppState, TexContext};

pub fn build_app<P: AsRef<Path>>(static_dir: P, tex: TexContext) -> Router {
    Router::new()
        .route("/", axum::routing::get(crate::router::index))
        .fallback_service(ServeDir::new(static_dir))
        .with_state(AppState { tex })
}

pub async fn bootstrap<A: ToSocketAddrs, P: AsRef<Path>>(
    address: A,
    static_dir: P,
    tex: TexContext,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_app(static_dir, tex);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
