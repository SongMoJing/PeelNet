#[macro_use]
extern crate rust_i18n;

use std::fs;
use directories::BaseDirs;
use global::config::Config;
use global::io::Log;
use lazy_static::lazy_static;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::OnceLock;

i18n!("locales", fallback = "zh-CN");

lazy_static! {
    static ref VERTION: &'static str = env!("CARGO_PKG_VERSION");
    static ref PATH_FILE_CONFIG: PathBuf = BaseDirs::new().map(|base|
        // Windows: ~AppData\Roaming\peel_net\core_server.toml
        // Linux  : ~/.config/peel_net/core_server.toml
        // macOS  : ~/Library/Application Support/peel_net/core_server.toml
        base.config_dir().join("peel_net").join("core_server").join("core_server.toml")
    ).unwrap_or_else(|| {
        Log::e("error", t!("error.path_invalid")).print();
        exit(1);
    });
    static ref PATH_DIR_LOG: PathBuf = BaseDirs::new().map(|base| {
        // Windows: ~\AppData\Local\peel_net\logs
        // Linux  : ~/.local/share/peel_net/logs
        // macOS  : ~/Library/Application Support/peel_net/logs
        base.data_local_dir().join("peel_net").join("core_server").join("logs")
    }).unwrap_or_else(|| {
        Log::e("error", t!("error.path_invalid")).print();
        exit(1);
    });
}

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() {
    rust_i18n::set_locale(&std::env::var("LANG").unwrap_or_else(|_| "zh-CN".to_string()));
    check_config();
    print_hello();
    if let Some(config) = CONFIG.get() {
        controller::start_controller_service(config);
        if let Some(net_controller) = &config.controller.network_controller {
            controller::start_net_controller_service(net_controller, &config.controller.ca, &config.controller.jwt).await;
        }
        if let Some(web_ui) = &config.controller.web_ui {
            web_ui::start_web_ui_service(web_ui, &config.controller.ca, &config.controller.jwt);
        }
    } else {
        Log::i(t!("tag.read_config"), t!("error.unexpected_error")).print();
        exit(1);
    }
}

fn check_config() {
    fn load_config(path: &Path) -> Result<Config, Error> {
        if !path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "config file not found"));
        }
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content).map_err(|e| Error::new(ErrorKind::InvalidData, e))?)
    }

    match load_config(PATH_FILE_CONFIG.as_path()) {
        Ok(config) => {
            CONFIG.set(config).unwrap_or_else(|_| {
                Log::i(t!("tag.read_config"), t!("error.unexpected_error")).print();
                exit(1);
            });
            Log::i(t!("tag.read_config"), t!("info.config_load_success")).print();
        }
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    Log::e(
                        t!("tag.read_config"),
                        t!(
                            "error.not_found",
                            ath = PATH_FILE_CONFIG.as_path().to_str().unwrap_or("None")
                        ),
                    )
                    .print();
                }
                ErrorKind::PermissionDenied => {
                    Log::e(t!("tag.read_config"), t!("error.permission_denied")).print();
                }
                ErrorKind::InvalidData => {
                    Log::e(
                        t!("tag.read_config"),
                        t!(
                            "error.invalid_data",
                            path = PATH_FILE_CONFIG.as_path().to_str().unwrap_or("None")
                        ),
                    )
                    .print();
                    Log::i(t!("tag.read_config"), err.to_string()).print();
                }
                e => {
                    println!("{:?}", e);
                    Log::e(t!("tag.read_config"), t!("error.unexpected_error")).print();
                }
            }
            exit(1);
        }
    }
}

fn print_hello() {
    let peel_server = r#"
██╗    ██╗███████╗██╗      ██████╗ ██████╗ ███╗   ███╗███████╗    ████████╗ ██████╗
██║    ██║██╔════╝██║     ██╔════╝██╔═══██╗████╗ ████║██╔════╝    ╚══██╔══╝██╔═══██╗
██║ █╗ ██║█████╗  ██║     ██║     ██║   ██║██╔████╔██║█████╗         ██║   ██║   ██║
██║███╗██║██╔══╝  ██║     ██║     ██║   ██║██║╚██╔╝██║██╔══╝         ██║   ██║   ██║
╚███╔███╔╝███████╗███████╗╚██████╗╚██████╔╝██║ ╚═╝ ██║███████╗       ██║   ╚██████╔╝
 ╚══╝╚══╝ ╚══════╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝       ╚═╝    ╚═════╝

██████╗ ███████╗███████╗██╗         ███████╗███████╗██████╗ ██╗   ██╗███████╗██████╗
██╔══██╗██╔════╝██╔════╝██║         ██╔════╝██╔════╝██╔══██╗██║   ██║██╔════╝██╔══██╗
██████╔╝█████╗  █████╗  ██║         ███████╗█████╗  ██████╔╝██║   ██║█████╗  ██████╔╝
██╔═══╝ ██╔══╝  ██╔══╝  ██║         ╚════██║██╔══╝  ██╔══██╗╚██╗ ██╔╝██╔══╝  ██╔══██╗
██║     ███████╗███████╗███████╗    ███████║███████╗██║  ██║ ╚████╔╝ ███████╗██║  ██║
╚═╝     ╚══════╝╚══════╝╚══════╝    ╚══════╝╚══════╝╚═╝  ╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝
"#
    .trim();
    Log::i(
        t!("tag.version"),
        format!("Peel Server v{}", (&VERTION).to_string()),
    )
    .print();
    Log::i(t!("tag.author"), t!("info.author")).print();
    Log::i(t!("tag.welcome"), peel_server).print();
}
