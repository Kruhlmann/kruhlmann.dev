use crate::TexContextLoadError;

pub const SERVER_HOST_KEY: &str = "SERVER_HOST";
pub const SERVER_PORT_KEY: &str = "SERVER_PORT";
pub const STATIC_DIR_KEY: &str = "STATIC_DIR";
pub const TEX_DIR_KEY: &str = "TEX_DIR";

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("environment variable '{key}': {source}")]
    MissingVar {
        key: &'static str,
        #[source]
        source: std::env::VarError,
    },
    #[error("environment variable '{key}' invalid: {source}")]
    InvalidVar {
        key: &'static str,
        #[source]
        source: std::env::VarError,
    },
    #[error("invalid port: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
    #[error("tex context load failure: {0}")]
    InvalidTexDirectory(#[from] TexContextLoadError),
}

impl ConfigError {
    pub fn from_var_error(key: &'static str, source: std::env::VarError) -> Self {
        match source {
            std::env::VarError::NotPresent => ConfigError::MissingVar { key, source },
            std::env::VarError::NotUnicode(..) => ConfigError::MissingVar { key, source },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub static_dir: String,
    pub tex_dir: String,
}

impl Config {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn from_environment<F>(read_env: F) -> Result<Self, ConfigError>
    where
        F: Fn(&str) -> Result<String, std::env::VarError>,
    {
        let host = read_env(SERVER_HOST_KEY)
            .map_err(|e| ConfigError::from_var_error(SERVER_HOST_KEY, e))?;
        let port = read_env(SERVER_PORT_KEY)
            .map_err(|e| ConfigError::from_var_error(SERVER_PORT_KEY, e))?
            .parse()?;
        let static_dir =
            read_env(STATIC_DIR_KEY).map_err(|e| ConfigError::from_var_error(STATIC_DIR_KEY, e))?;
        let tex_dir =
            read_env(TEX_DIR_KEY).map_err(|e| ConfigError::from_var_error(TEX_DIR_KEY, e))?;

        Ok(Self {
            host,
            port,
            static_dir,
            tex_dir,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reads_environment() {
        let read_env = |k: &str| match k {
            SERVER_HOST_KEY => Ok("0.0.0.0".to_string()),
            SERVER_PORT_KEY => Ok("3000".to_string()),
            STATIC_DIR_KEY => Ok(".".to_string()),
            TEX_DIR_KEY => Ok(".".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        };
        let res = Config::from_environment(read_env);
        assert!(res.is_ok())
    }

    #[test]
    fn test_fails_on_missing_var() {
        let read_env = |_: &str| Err(std::env::VarError::NotPresent);
        let res = Config::from_environment(read_env);
        assert!(res.is_err())
    }

    #[test]
    fn test_fails_on_invalid_utf8_var() {
        let read_env = |_: &str| Err(std::env::VarError::NotUnicode("".into()));
        let res = Config::from_environment(read_env);
        assert!(res.is_err())
    }

    #[test]
    fn test_fails_on_invalid_port_var() {
        let read_env = |k: &str| match k {
            SERVER_HOST_KEY => Ok("0.0.0.0".to_string()),
            SERVER_PORT_KEY => Ok("a".to_string()),
            STATIC_DIR_KEY => Ok(".".to_string()),
            TEX_DIR_KEY => Ok(".".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        };
        let res = Config::from_environment(read_env);
        assert!(res.is_err())
    }
}
