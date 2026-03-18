use std::process::exit;
use rust_i18n::t;
use global::config::NetworkControllerConfig;
use global::io::Log;

pub fn start_net_controller_service(config: &NetworkControllerConfig) {
	// Log::i(t!("tag.read_config"), t!("error.unexpected_error")).print();
	exit(1);
}