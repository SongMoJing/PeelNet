use rust_i18n::{i18n, t};
use global::config::{Config, NetworkControllerConfig};
use global::io::Log;

i18n!("locales", fallback = "zh-CN");

pub fn start_controller_service(config: &Config) {
	Log::i(t!("tag.netController_service"), t!("log.enable")).print();
}

pub fn start_net_controller_service(config: &NetworkControllerConfig) {
	Log::i(t!("tag.netController_service"), t!("log.netService.enable_ca")).print();
	Log::i(t!("tag.netController_service"), t!("log.netService.enable_authorizedKeys")).print();

	Log::i(t!("tag.netController_service"), t!("log.netService.enable", addr = config.addr, port = config.port)).print();
}
