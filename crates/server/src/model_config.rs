use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ServerConfig {
    feature: ServerFeatureConfig,
    wui: Option<WuiConfig>,
    tui: Option<TuiConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ServerFeatureConfig {
    webui: bool,
    tui: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct WuiConfig {
    port: u16,
    addr: Vec<String>,
    ca: CAConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TuiConfig {
    feature: TuiFeatureConfig,
    port: u16,
    addr: Vec<String>,
    ca: Option<CAConfig>,
    authorized_keys: Option<AuthorizedKeysConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TuiFeatureConfig {
    ca: bool,
    authorized_keys: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CAConfig {
    cert_ca: String,
    cert_server: String,
    cert_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorizedKeysConfig {
    authorized_keys: Vec<AuthorizedMap>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorizedMap {
    user: String,
    keys: String,
}

pub(crate) fn load_config(path: &Path) -> Result<Config, Error> {
    if !path.exists() {
	    return Err(Error::new(ErrorKind::NotFound, "config file not found"));
    }
    let content = fs::read_to_string(path)?;
    let mut config: Config = toml::from_str(&content).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    validate_and_trim(&mut config)?;
    Ok(config)
}

pub fn validate_and_trim(config: &mut Config) -> Result<(), Error> {
    let server = &mut config.server;
    if server.feature.webui {
        if server.wui.is_none() {
            return Err(Error::new(ErrorKind::InvalidData, t!("error.feature_not_enabled", feature = "Web UI Service")));
        }
    } else {
        server.wui = None;
    }
    if server.feature.tui {
        if let Some(tui) = &mut server.tui {
            if tui.feature.ca && tui.ca.is_none() {
                return Err(Error::new(ErrorKind::InvalidData, t!("error.feature_not_enabled", feature = "TUI Certificate Authority")));
            } else {
                tui.ca = None;
            }
            if tui.feature.authorized_keys && tui.authorized_keys.is_none() {
                return Err(Error::new(ErrorKind::InvalidData, t!("error.feature_not_enabled", feature = "TUI Authorized Keys")));
            } else {
                tui.authorized_keys = None;
            }
        } else {
            return Err(Error::new(ErrorKind::InvalidData, t!("error.feature_not_enabled", feature = "Terminal UI Service")));
        }
    } else {
        server.tui = None;
    }
    Ok(())
}
