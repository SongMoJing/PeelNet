mod model;
mod pages;

use crate::model::config::Config;
use crate::model::saved::{ConnectInfo, Saved};
use crate::model::state::State;
use crate::pages::index::IndexPage;
use crate::pages::menu::set_menu;
use crate::pages::Page;
use cursive::style::{Color, PaletteColor};
use cursive::theme::Theme;
use std::rc::Rc;

fn main() {
    let config = Rc::new(load_config());
    let state = Rc::new(load_state(&config));
    let saved = Rc::new(load_saved(&config));
    let mut siv = cursive::default();
    let mut theme = Theme::default();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::TerminalDefault;
    theme.palette[PaletteColor::Primary] = Color::Rgb(235, 255, 255);
    theme.palette[PaletteColor::TitlePrimary] = Color::Rgb(127, 255, 212);
    theme.palette[PaletteColor::Highlight] = Color::Rgb(173, 217, 226);
    theme.palette[PaletteColor::HighlightText] = Color::Rgb(100, 149, 237);
    theme.shadow = false;
    siv.set_theme(theme);
    siv.add_layer(IndexPage::new().render(config, state, saved));
    set_menu(&mut siv);
    siv.run();
}

fn load_config() -> Config {
    Config {}
}

fn load_state(config: &Config) -> State {
    State {}
}

fn load_saved(config: &Config) -> Saved {
    let mut saved = Saved::new();
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接2".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接3".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接4".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接5".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接6".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接7".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接8".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接2".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接3".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接4".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接5".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接6".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接7".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接8".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接2".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接3".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接4".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接5".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接6".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接7".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved.add_connect_info(
        ConnectInfo {
            name: "测试连接8".into(),
            host: "127.0.0.1".into(),
            port: 3310,
            cert: "None".into(),
            key: "None".into(),
        }
    );
    saved
}
