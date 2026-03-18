use global::config::{Config, WebUIConfig};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ConfigContent {
    pub(crate) server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ServerConfig {
    feature: ServerFeatureConfig,
    pub(crate) wui: Option<WuiConfig>,
    pub(crate) tui: Option<TuiConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ServerFeatureConfig {
    webui: bool,
    tui: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct WuiConfig {
    pub(crate) port: u16,
    pub(crate) addr: String,
    pub(crate) ca: CAConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TuiConfig {
    feature: TuiFeatureConfig,
    pub(crate) port: u16,
    pub(crate) addr: String,
    pub(crate) ca: Option<CAConfig>,
    pub(crate) authorized_keys: Option<AuthorizedKeysConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TuiFeatureConfig {
    ca: bool,
    authorized_keys: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CAConfig {
    pub(crate) cert_ca: String,
    pub(crate) cert_server: String,
    pub(crate) cert_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorizedKeysConfig {
    pub(crate) authorized_keys: Vec<AuthorizedMap>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AuthorizedMap {
    pub(crate) user: String,
    pub(crate) keys: String,
}

impl ConfigContent {
    fn parse(self) -> Config {
        Config {
            server: global::config::ServerConfig {
                web_ui: if let Some(wui) = self.server.wui {
                    Some(WebUIConfig {
                        port: wui.port,
                        addr: wui.addr,
                        ca: global::config::CAConfig {
                            cert_ca: wui.ca.cert_ca,
                            cert_server: wui.ca.cert_server,
                            cert_key: wui.ca.cert_key,
                        },
                    })
                } else {
                    None
                },
                network_controller: if let Some(tui) = self.server.tui {
                    Some(global::config::NetworkControllerConfig {
                        port: tui.port,
                        addr: tui.addr,
                        ca: if let Some(ca) = tui.ca {
                            Some(global::config::CAConfig {
                                cert_ca: ca.cert_ca,
                                cert_server: ca.cert_server,
                                cert_key: ca.cert_key,
                            })
                        } else {
                            None
                        },
                        authorized_keys: if let Some(authorized_keys) = tui.authorized_keys {
                            let mut auth_keys = vec![];
                            for authorized_key in authorized_keys.authorized_keys {
                                auth_keys.push(global::config::AuthorizedMap {
                                    user: authorized_key.user,
                                    keys: authorized_key.keys,
                                });
                            }
                            Some(global::config::AuthorizedKeysConfig {
                                authorized_keys: auth_keys
                            })
                        } else {
                            None
                        },
                    })
                } else {
                    None
                },
            },
        }
    }
}

pub(crate) fn load_config(path: &Path) -> Result<Config, Error> {
    if !path.exists() {
        return Err(Error::new(ErrorKind::NotFound, "config file not found"));
    }
    let content = fs::read_to_string(path)?;
    let mut config: ConfigContent =
        toml::from_str(&content).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    validate_and_trim(&mut config)?;
    Ok(config.parse())
}

pub fn validate_and_trim(config: &mut ConfigContent) -> Result<(), Error> {
    let server = &mut config.server;
    if server.feature.webui {
        if server.wui.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                t!("error.feature_not_enabled", feature = "Web UI Service"),
            ));
        }
    } else {
        server.wui = None;
    }
    if server.feature.tui {
        if let Some(tui) = &mut server.tui {
            if tui.feature.ca && tui.ca.is_none() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    t!(
                        "error.feature_not_enabled",
                        feature = "TUI Certificate Authority"
                    ),
                ));
            } else {
                tui.ca = None;
            }
            if tui.feature.authorized_keys && tui.authorized_keys.is_none() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    t!("error.feature_not_enabled", feature = "TUI Authorized Keys"),
                ));
            } else {
                tui.authorized_keys = None;
            }
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                t!("error.feature_not_enabled", feature = "Terminal UI Service"),
            ));
        }
    } else {
        server.tui = None;
    }
    Ok(())
}
