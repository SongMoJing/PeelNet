mod pages;
mod status;

use cursive::view::Resizable;
use cursive::style::{Color, PaletteColor};
use cursive::theme::Theme;
use crate::pages::index::IndexPage;
use crate::pages::Page;

fn main() {
    let mut siv = cursive::default();
    let mut theme = Theme::default();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::TerminalDefault;
    theme.palette[PaletteColor::Primary] = Color::TerminalDefault;
    theme.shadow = false;
    siv.set_theme(theme);
    IndexPage::new().render(&mut siv);
    siv.run();
}