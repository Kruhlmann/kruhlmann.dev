use kruhlmann_dev::{
    TexContextLoadError,
    config::{Config, ConfigError, SERVER_HOST_KEY, SERVER_PORT_KEY, STATIC_DIR_KEY, TEX_DIR_KEY},
};
use tracing_subscriber::prelude::*;

const DEFAULT_LOG_FILTER: &str = "debug";
const DEBUG_SERVER_HOST: &str = "127.0.0.1";
const DEBUG_SERVER_PORT: &str = "3000";
const DEBUG_STATIC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../static");
const DEBUG_TEX_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tex");

#[tokio::main]
async fn main() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| DEFAULT_LOG_FILTER.into());
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    if cfg!(debug_assertions) {
        tracing::warn!(
            ?DEBUG_SERVER_HOST,
            ?DEBUG_SERVER_PORT,
            "running with pre-defined SERVER_HOST and SERVER_PORT in debug mode"
        );
        unsafe {
            std::env::set_var(SERVER_HOST_KEY, DEBUG_SERVER_HOST);
            std::env::set_var(SERVER_PORT_KEY, DEBUG_SERVER_PORT);
            std::env::set_var(STATIC_DIR_KEY, DEBUG_STATIC_DIR);
            std::env::set_var(TEX_DIR_KEY, DEBUG_TEX_DIR);
        }
    }

    let config = Config::from_environment()
        .inspect_err(|error: &ConfigError| tracing::error!(?error, "invalid configuration"))
        .unwrap();
    let tex_context = kruhlmann_dev::tex::TexContext::load_from_directory(&config.tex_dir)
        .inspect_err(|error: &TexContextLoadError| tracing::error!(?error, "invalid tex context"))
        .unwrap();
    let static_dir = config.static_dir.clone();
    match std::fs::exists(&static_dir) {
        Ok(true) => {}
        Ok(false) => tracing::error!(?static_dir, "static dir does not exist"),
        Err(error) => tracing::error!(?static_dir, ?error, "static dir error"),
    }

    let address = config.address();
    tracing::info!(?address, ?static_dir, "bootstrap server");

    kruhlmann_dev::bootstrap(address, static_dir, tex_context)
        .await
        .inspect_err(|error| tracing::error!(?error, "unrecoverable server error"))
        .unwrap();
}
