#[macro_use]
extern crate rust_i18n;

use std::io::ErrorKind;
use crate::io::Log;
use directories::BaseDirs;
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::OnceLock;
use crate::model_config::Config;

mod io;
mod model_config;

i18n!("locales", fallback = "zh-CN");

lazy_static! {
    static ref VERTION: &'static str = env!("CARGO_PKG_VERSION");
    static ref PATH_FILE_CONFIG: PathBuf = BaseDirs::new().map(|base|
        // Windows: ~AppData\Roaming\peel_net\server.toml
        // Linux  : ~/.config/peel_net/server.toml
        // macOS  : ~/Library/Application Support/peel_net/server.toml
        base.config_dir().join("peel_net").join("server").join("server.toml")
    ).unwrap_or_else(|| {
        Log::e("error", t!("error.path_invalid")).print();
        std::process::exit(1);
    });
    static ref PATH_DIR_LOG: PathBuf = BaseDirs::new().map(|base| {
        // Windows: ~\AppData\Local\peel_net\logs
        // Linux  : ~/.local/share/peel_net/logs
        // macOS  : ~/Library/Application Support/peel_net/logs
        base.data_local_dir().join("peel_net").join("server").join("logs")
    }).unwrap_or_else(|| {
        Log::e("error", t!("error.path_invalid")).print();
        std::process::exit(1);
    });
}

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() {
    rust_i18n::set_locale(&std::env::var("LANG").unwrap_or_else(|_| "zh-CN".to_string()));
    check_config();
    print_hello();
}

fn check_config() {
    match model_config::load_config(PATH_FILE_CONFIG.as_path()) {
        Ok(config) => {
            CONFIG.set(config).unwrap_or_else(|_| {
                Log::i(t!("tag.read_config"), t!("error.unexpected_error")).print();
            });
            Log::i(t!("tag.read_config"), t!("info.config_load_success")).print();
        }
        Err(err) => {
            match err.kind() {
                ErrorKind::PermissionDenied => {
                    Log::e(t!("tag.read_config"), t!("error.permission_denied")).print();
                    std::process::exit(1);
                }
                ErrorKind::InvalidData => {
                    Log::e(t!("tag.read_config"), t!("error.invalid_data", path = PATH_FILE_CONFIG.as_path().to_str().unwrap_or("None"))).print();
                    std::process::exit(1);
                }
                e => {
                    println!("{:?}", e);
                    Log::e(t!("tag.read_config"), t!("error.unexpected_error")).print();
                    std::process::exit(1);
                }
            }
        }
    }
}

fn print_hello() {
    let tag = "Welcome to Peel Server";
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
"#.trim();
    Log::i("version", format!("Peel Server v{}", (&VERTION).to_string())).print();
    Log::i("author", t!("info.author")).print();
    Log::i(tag, peel_server).print();
}
