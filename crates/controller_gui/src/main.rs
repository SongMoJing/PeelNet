mod component;

use iced::advanced::Widget;
use iced::widget::{button, column, container, mouse_area, row, rule, stack, text}; // 确保这里是小写的 rule
use iced::window::{icon, Icon, Settings};
use iced::{Alignment, Element, Length, Padding, Theme};
use crate::component::menu::{Menu, MenuItem};

fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view)
        .title("Iced App")
        .window(Settings {
            // 保留你的图标
            icon: Some(Icon::from(
                icon::from_file_data(include_bytes!("../res/icon.ico"), None).unwrap(),
            )),
            ..Default::default()
        })
        .theme(get_theme) // 使用函数指针，彻底解决生命周期 (FnOnce) 报错
        .run()
}

// 解决编译器的 FnOnce / Fn 生命周期推导 Bug
fn get_theme(_state: &AppState) -> Theme {
    Theme::Dark
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuType {
    None,
    File,
    Connect,
    Help,
}

struct AppState {
    tabs: Vec<String>,
    active_tab_index: usize,
    active_menu: MenuType,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tabs: vec!["开始".to_string()],
            active_tab_index: 0,
            active_menu: MenuType::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(usize),
    CloseTab(usize),
    ToggleMenu(MenuType),
    MenuAction(&'static str),
}

impl AppState {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(index) => self.active_tab_index = index,
            Message::CloseTab(index) => {
                if self.tabs.len() > 1 {
                    self.tabs.remove(index);
                    if self.active_tab_index >= self.tabs.len() {
                        self.active_tab_index = self.tabs.len().saturating_sub(1);
                    }
                }
            }
            Message::ToggleMenu(menu) => {
                self.active_menu = if self.active_menu == menu {
                    MenuType::None
                } else {
                    menu
                };
            }
            Message::MenuAction(action) => {
                if action == "NewConnect" {
                    self.tabs.push(format!("连接 {}", self.tabs.len()));
                    self.active_tab_index = self.tabs.len() - 1;
                }
                self.active_menu = MenuType::None;
            }
        }
    }

    fn get_active_menu_data(&self) -> Option<Menu> {
        match self.active_menu {
            MenuType::File => Some(Menu::new(vec![
                MenuItem::Item { enable: true, name: "打开文件".into(), action: "OpenFile", child: None },
                MenuItem::Item { enable: true, name: "保存".into(), action: "SaveFile", child: None },
                MenuItem::Separator,
                MenuItem::Item { enable: false, name: "退出".into(), action: "Exit", child: None },
            ])),
            MenuType::Connect => Some(Menu::new(vec![
                MenuItem::Item { enable: true, name: "新建连接".into(), action: "NewConnect", child: None },
            ])),
            MenuType::Help => Some(Menu::new(vec![
                MenuItem::Item { enable: true, name: "关于".into(), action: "About", child: None },
            ])),
            MenuType::None => None,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // 顶部菜单按钮
        let menu_bar = row![
            button(text("文件").size(14))
                .on_press(Message::ToggleMenu(MenuType::File))
                .style(button::text),
            button(text("连接").size(14))
                .on_press(Message::ToggleMenu(MenuType::Connect))
                .style(button::text),
            button(text("帮助").size(14))
                .on_press(Message::ToggleMenu(MenuType::Help))
                .style(button::text),
        ]
        .spacing(10)
        .padding(5);

        // 动态 Tab 栏
        let mut tab_bar = row![].spacing(5).padding(10).align_y(Alignment::Center);
        for (i, title) in self.tabs.iter().enumerate() {
            let is_active = i == self.active_tab_index;
            let mut tab_item = container(
                row![
                    button(text(title).size(14))
                        .on_press(Message::TabSelected(i))
                        .style(button::text),
                    button(text("×").size(12))
                        .on_press(Message::CloseTab(i))
                        .style(button::text)
                ]
                .align_y(Alignment::Center),
            );
            if is_active {
                tab_item = tab_item.style(container::primary);
            }
            tab_bar = tab_bar.push(tab_item);
        }

        // 主页面布局
        let main_layout = column![
            menu_bar,
            rule::horizontal(1),
            tab_bar,
            container(
                text(format!(
                    "当前页面内容: {}",
                    self.tabs[self.active_tab_index]
                ))
                .size(20)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
        ];

        // 菜单 PopupWindow 逻辑
        if self.active_menu != MenuType::None {
            let overlay = mouse_area(container(column![]).width(Length::Fill).height(Length::Fill))
                .on_press(Message::ToggleMenu(MenuType::None))
                .interaction(iced::mouse::Interaction::Idle);

            // 获取偏移量
            let x_offset = match self.active_menu {
                MenuType::File => 5.0,
                MenuType::Connect => 60.0,
                MenuType::Help => 115.0,
                _ => 0.0,
            };

            // 使用 Menu::view 渲染，并传入如何将 action 转成 Message
            let menu_content = if let Some(menu_data) = self.get_active_menu_data() {
                menu_data.view(Message::MenuAction)
            } else {
                column![].into()
            };

            let menu_popup = container(menu_content)
                .padding(Padding {
                    top: 35.0,
                    right: 0.0,
                    bottom: 0.0,
                    left: x_offset,
                });

            stack![main_layout, overlay, menu_popup].into()
        } else {
            main_layout.into()
        }
    }
}
