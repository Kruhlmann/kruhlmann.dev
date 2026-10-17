use tracing_subscriber::prelude::*;

const DEFAULT_LOG_FILTER: &'static str = "debug";
const DEBUG_SERVER_HOST: &'static str = "127.0.0.1";
const DEBUG_SERVER_PORT: &'static str = "3000";
const DEBUG_STATIC_DIR: &'static str = "./static";
const SERVER_HOST_KEY: &'static str = "SERVER_HOST";
const SERVER_PORT_KEY: &'static str = "SERVER_PORT";
const STATIC_DIR_KEY: &'static str = "STATIC_DIR";

fn getenv(name: &str) -> Result<String, std::env::VarError> {
    std::env::var(name)
        .inspect_err(|error| tracing::error!(?name, ?error, "missing environment variable"))
}

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
        }
    }

    let host = getenv(SERVER_HOST_KEY).unwrap();
    let port = getenv(SERVER_PORT_KEY).unwrap();
    let port: u16 = port
        .parse()
        .inspect_err(|error| tracing::error!(?error, ?port, "invalid port value"))
        .unwrap();
    let static_dir = getenv(STATIC_DIR_KEY).unwrap();
    match std::fs::exists(&static_dir) {
        Ok(true) => {}
        Ok(false) => tracing::error!(?static_dir, "static dir does not exist"),
        Err(error) => tracing::error!(?static_dir, ?error, "static dir error"),
    }

    let address = format!("{host}:{port}");
    tracing::info!(?address, ?static_dir, "bootstrap server");

    kruhlmann_dev::bootstrap(address, static_dir)
        .await
        .inspect_err(|error| tracing::error!(?error, "unrecoverable server error"))
        .unwrap();
}