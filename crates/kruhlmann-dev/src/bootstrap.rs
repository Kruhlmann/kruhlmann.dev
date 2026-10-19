use std::path::Path;

use axum::Router;
use tokio::net::ToSocketAddrs;
use tower_http::services::ServeDir;

use crate::{AppState, TexContext};

async fn shutdown_signal() {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install SIGTERM handler");

    tokio::select! {
        result = tokio::signal::ctrl_c() => {
            result.expect("failed to listen for Ctrl+C");
        }
        _ = terminate.recv() => {}
    }

    tracing::info!("shutting down");
}

pub fn build_app<P: AsRef<Path>>(static_dir: P, tex: TexContext) -> Router {
    Router::new()
        .route("/", axum::routing::get(crate::router::index))
        .fallback_service(ServeDir::new(static_dir))
        .with_state(AppState {
            tex,
            page_cache: Default::default(),
        })
}

pub async fn bootstrap<A: ToSocketAddrs, P: AsRef<Path>>(
    address: A,
    static_dir: P,
    tex: TexContext,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_app(static_dir, tex);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
