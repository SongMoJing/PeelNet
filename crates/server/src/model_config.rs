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
	wui: bool,
	tui: bool,
	web_ui: WuiConfig,
	terminal_ui: TuiConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct WuiConfig {
	port: u16,
	addr: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TuiConfig {
	port: u16,
	addr: Vec<String>,
}

pub(crate) fn load_config(path: &Path) -> Result<Config, Error> {
	if !path.exists() {
		let default_config = Config {
			server: ServerConfig {
				wui: true,
				tui: true,
				web_ui: WuiConfig {
					port: 9771,
					addr: vec!["127.0.0.1".to_string()],
				},
				terminal_ui: TuiConfig {
					port: 9772,
					addr: vec!["127.0.0.1".to_string()],
				},
			},
		};
		let toml_str = toml::to_string_pretty(&default_config).unwrap();
		fs::write(path, toml_str)?;
		return Ok(default_config);
	}
	let content = fs::read_to_string(path)?;
	Ok(toml::from_str(&content).map_err(|e| Error::new(ErrorKind::InvalidData, e))?)
}
