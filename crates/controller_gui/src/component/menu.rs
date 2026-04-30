// menu.rs
use iced::widget::{button, column, container, rule, text};
use iced::{Alignment, Element, Length, Theme};

pub struct Menu {
    pub items: Vec<MenuItem>,
}

pub enum MenuItem {
    /// 分割线
    Separator,
    /// 菜单项
    Item {
        enable: bool,
        name: String,
        action: &'static str,
        child: Option<Box<Menu>>,
    },
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self { items }
    }

    pub fn view<Message>(&self, on_action: impl Fn(&'static str) -> Message + Copy) -> Element<'_, Message>
    where
        Message: Clone + 'static
    {
        let mut content = column![].spacing(2);

        for item in &self.items {
            match item {
                MenuItem::Separator => {
                    content = content.push(rule::horizontal(1));
                }
                MenuItem::Item { enable, name, action, child: _ } => {
                    let mut btn = button(
                        text(name).size(14)
                    )
                        .width(150)
                        .style(button::text);

                    if *enable {
                        btn = btn.on_press(on_action(action));
                    }

                    content = content.push(btn);
                }
            }
        }

        container(content)
            .padding(5)
            .style(container::secondary)
            .into()
    }
}