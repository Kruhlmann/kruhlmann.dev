use std::env::VarError;

pub const SERVER_HOST_KEY: &str = "SERVER_HOST";
pub const SERVER_PORT_KEY: &str = "SERVER_PORT";
pub const STATIC_DIR_KEY: &str = "STATIC_DIR";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub static_dir: String,
}

impl Config {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    MissingVar { name: String, reason: String },
    InvalidPort { value: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingVar { name, reason } => {
                write!(f, "missing environment variable {name}: {reason}")
            }
            ConfigError::InvalidPort { value } => write!(f, "invalid port value: {value}"),
        }
    }
}

impl std::error::Error for ConfigError {}

fn missing(name: &str, error: VarError) -> ConfigError {
    ConfigError::MissingVar {
        name: name.to_owned(),
        reason: error.to_string(),
    }
}

pub fn parse_config(
    getenv: impl Fn(&str) -> Result<String, VarError>,
) -> Result<Config, ConfigError> {
    let host = getenv(SERVER_HOST_KEY).map_err(|error| missing(SERVER_HOST_KEY, error))?;
    let port = getenv(SERVER_PORT_KEY).map_err(|error| missing(SERVER_PORT_KEY, error))?;
    let port: u16 = port
        .parse()
        .map_err(|_| ConfigError::InvalidPort { value: port })?;
    let static_dir = getenv(STATIC_DIR_KEY).map_err(|error| missing(STATIC_DIR_KEY, error))?;
    Ok(Config {
        host,
        port,
        static_dir,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Result<String, VarError> {
        let owned: Vec<(String, String)> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |name: &str| {
            owned
                .iter()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.clone())
                .ok_or(VarError::NotPresent)
        }
    }

    fn full_env() -> Vec<(&'static str, &'static str)> {
        vec![
            (SERVER_HOST_KEY, "0.0.0.0"),
            (SERVER_PORT_KEY, "8080"),
            (STATIC_DIR_KEY, "./static"),
        ]
    }

    #[test]
    fn parses_complete_environment() {
        let config = parse_config(env(&full_env())).unwrap();
        assert_eq!(
            config,
            Config {
                host: "0.0.0.0".to_string(),
                port: 8080,
                static_dir: "./static".to_string(),
            }
        );
    }

    #[test]
    fn address_formats_host_and_port() {
        let config = parse_config(env(&full_env())).unwrap();
        assert_eq!(config.address(), "0.0.0.0:8080");
    }

    #[test]
    fn accepts_port_boundaries() {
        for port in ["0", "65535"] {
            let mut e = full_env();
            e[1].1 = port;
            assert_eq!(
                parse_config(env(&e)).unwrap().port,
                port.parse::<u16>().unwrap()
            );
        }
    }

    #[test]
    fn rejects_missing_host() {
        let e = vec![(SERVER_PORT_KEY, "80"), (STATIC_DIR_KEY, "./static")];
        let err = parse_config(env(&e)).unwrap_err();
        assert!(matches!(
            err,
            ConfigError::MissingVar { name, .. } if name == SERVER_HOST_KEY
        ));
    }

    #[test]
    fn rejects_missing_port() {
        let e = vec![(SERVER_HOST_KEY, "127.0.0.1"), (STATIC_DIR_KEY, "./static")];
        let err = parse_config(env(&e)).unwrap_err();
        assert!(matches!(
            err,
            ConfigError::MissingVar { name, .. } if name == SERVER_PORT_KEY
        ));
    }

    #[test]
    fn rejects_missing_static_dir() {
        let e = vec![(SERVER_HOST_KEY, "127.0.0.1"), (SERVER_PORT_KEY, "80")];
        let err = parse_config(env(&e)).unwrap_err();
        assert!(matches!(
            err,
            ConfigError::MissingVar { name, .. } if name == STATIC_DIR_KEY
        ));
    }

    #[test]
    fn rejects_non_numeric_port() {
        let mut e = full_env();
        e[1].1 = "not-a-port";
        let err = parse_config(env(&e)).unwrap_err();
        assert_eq!(
            err,
            ConfigError::InvalidPort {
                value: "not-a-port".to_string()
            }
        );
    }

    #[test]
    fn rejects_out_of_range_port() {
        let mut e = full_env();
        e[1].1 = "65536";
        assert!(matches!(
            parse_config(env(&e)).unwrap_err(),
            ConfigError::InvalidPort { .. }
        ));
    }

    #[test]
    fn rejects_negative_port() {
        let mut e = full_env();
        e[1].1 = "-1";
        assert!(matches!(
            parse_config(env(&e)).unwrap_err(),
            ConfigError::InvalidPort { .. }
        ));
    }

    #[test]
    fn error_display_mentions_problem() {
        let err = parse_config(env(&[])).unwrap_err();
        assert!(err.to_string().contains(SERVER_HOST_KEY));
        let err = ConfigError::InvalidPort {
            value: "x".to_string(),
        };
        assert!(err.to_string().contains("x"));
    }
}
