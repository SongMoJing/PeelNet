use crate::model::config::Config;
use crate::model::saved::Saved;
use crate::model::state::State;
use crate::pages::Page;
use cursive::align::{Align, HAlign, VAlign};
use cursive::style::{Color, ColorStyle, PaletteColor};
use cursive::view::Resizable;
use cursive::views::*;
use cursive::View;
use std::rc::Rc;

pub struct IndexPage {}

impl IndexPage {
    pub fn new() -> IndexPage {
        IndexPage {}
    }
}

impl Page for IndexPage {
    fn render(&mut self, config: Rc<Config>, state: Rc<State>, saved: Rc<Saved>) -> LinearLayout {
        let header = TextView::new(format!(
            "{}\n{}\n{}\n{}\n{}",
            r#"________          ______                     __ "#,
            r#"___  __ \____________  /        _____  ___ _/ /_"#,
            r#"__  /_/ /  _ \  _ \_  /        \  __ \/ _ \  __/"#,
            r#"_  ____//  __/  __/  /___  __  / / / /  __/ /_  "#,
            r#"/_/     \___/\___//_____/ /_/ /_/ /_/\___/\__/  "#
        ))
        .style(ColorStyle::new(
            PaletteColor::HighlightText,
            Color::TerminalDefault,
        ))
        .center();
        let mut connected_list = LinearLayout::vertical();
        for item in saved.list_connect_info() {
            connected_list.add_child(TextView::new(item.name));
        }
        let content = Panel::new(
            LinearLayout::vertical().child(
                Panel::new(ScrollView::new(connected_list))
                    .title("历史连接")
                    .min_width(50)
                    .min_height(8)
                    .max_height(12)
                    .max_width(50),
            ),
        );
        let monitor = LinearLayout::horizontal().child(TextView::new("未连接"));
        let layout = LinearLayout::vertical()
            .child(content.full_screen())
            .child(monitor);
        layout
    }
}
