mod net_server;

use core_global::config::{CAConfig, Config, JWTConfig, NetworkControllerConfig};
use core_global::io::Log;
use rust_i18n::{i18n, t};

i18n!("locales", fallback = "zh-CN");

pub fn start_controller_service(config: &Config) {
    Log::i(t!("tag.netController_service.self"), t!("log.enable")).print();
}

pub async fn start_net_controller_service(
    config: &NetworkControllerConfig,
    ca: &CAConfig,
    jwt: &JWTConfig,
) {
    match net_server::start_net_service(config.addr.clone(), config.port, ca, jwt).await {
        Ok(_handle) => {
            Log::i(
                "tag.netController_service.self",
                t!(
                    "log.netService.enable",
                    addr = config.addr,
                    port = config.port
                ),
            )
            .print();
        }
        Err(e) => {
            Log::e(
                "tag.netController_service.loadCA",
                t!("error.netService.start", err = e),
            )
            .print();
        }
    }
}
