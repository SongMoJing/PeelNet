use crate::model::config::Config;
use crate::model::saved::{ConnectInfo, Saved};
use crate::model::state::State;
use crate::pages::Page;
use cursive::view::{Nameable, Resizable};
use cursive::views::*;
use std::rc::Rc;
use unicode_width::UnicodeWidthStr;

pub struct IndexPage {
    config: Rc<Config>,
    state: Rc<State>,
    saved: Rc<Saved>,
}

impl IndexPage {
    pub fn new(config: Rc<Config>, state: Rc<State>, saved: Rc<Saved>) -> IndexPage {
        IndexPage {
            config,
            state,
            saved,
        }
    }
}

impl Page for IndexPage {
    fn build(&mut self) -> LinearLayout {
        let header = TextView::new(format!(
            "{}\n{}\n{}\n{}\n{}\n ",
            r#"________          ______                     __ "#,
            r#"___  __ \____________  /        _____  ___ _/ /_"#,
            r#"__  /_/ /  _ \  _ \_  /        \  __ \/ _ \  __/"#,
            r#"_  ____//  __/  __/  /___  __  / / / /  __/ /_  "#,
            r#"/_/     \___/\___//_____/ /_/ /_/ /_/\___/\__/  "#
        ))
        .center()
        .with_name("header");
        let content = Panel::new(
            create_def_panel(self.saved.clone())
                .full_screen()
                .with_name("content"),
        )
        .with_name("board");
        let monitor =
            LinearLayout::horizontal().child(TextView::new("未连接").with_name("monitor:target"));
        LinearLayout::vertical()
            .child(header)
            .child(content)
            .child(monitor)
    }
}

fn create_def_panel(saved: Rc<Saved>) -> LinearLayout {
    fn create_connection_list(saved: Rc<Saved>) -> SelectView<ConnectInfo> {
        let mut select = SelectView::new().autojump();
        for item in saved.list_connect_info() {
            let name = format_name(&item.name, 16);
            let label = format!("{} {}:{}@{}", name, item.host, item.port, item.user);
            select.add_item(label, item);
        }
        select.set_on_submit(move |_, info| {});
        select
    }

    let mut content = LinearLayout::horizontal().child(DummyView.full_width());
    let historical_connection = LinearLayout::vertical().child(
        Panel::new(ScrollView::new(create_connection_list(saved)))
            .title("历史连接")
            .min_width(50)
            .min_height(8)
            .max_height(12)
            .max_width(50),
    );
    content = content.child(historical_connection);
    content = content.child(DummyView.full_width());
    LinearLayout::vertical()
        .child(DummyView.full_height())
        .child(content)
        .child(DummyView.full_height())
}

fn format_name(name: &str, target_width: usize) -> String {
    let current_width = name.width();
    if current_width == target_width {
        name.to_string()
    } else if current_width > target_width {
        let mut res = String::new();
        let mut total_width = 0;
        let limit = target_width - 3;
        for c in name.chars() {
            let w = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
            if total_width + w > limit {
                break;
            }
            res.push(c);
            total_width += w;
        }
        res.push_str(&" ".repeat(limit - total_width));
        res.push_str("...");
        res
    } else {
        format!("{}{}", name, " ".repeat(target_width - current_width))
    }
}
