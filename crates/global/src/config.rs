use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
	pub web_ui: Option<WebUIConfig>,
	pub network_controller: Option<NetworkControllerConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebUIConfig {
	pub port: u16,
	pub addr: Vec<String>,
	pub ca: CAConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkControllerConfig {
	pub port: u16,
	pub addr: Vec<String>,
	pub ca: Option<CAConfig>,
	pub authorized_keys: Option<AuthorizedKeysConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CAConfig {
	pub cert_ca: String,
	pub cert_server: String,
	pub cert_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizedKeysConfig {
	pub authorized_keys: Vec<AuthorizedMap>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizedMap {
	pub user: String,
	pub keys: String,
}