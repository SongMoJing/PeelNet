use crate::pages::Page;
use cursive::align::Align;
use cursive::menu::Tree;
use cursive::traits::Nameable;
use cursive::view::Resizable;
use cursive::views::{
    BoxedView, Button, Dialog, LinearLayout, Panel, SelectView, StackView, TextView,
};
use cursive::CursiveRunnable;
use cursive_tabs::TabPanel;

pub struct IndexPage {}

impl IndexPage {
    pub fn new() -> IndexPage {
        IndexPage {}
    }
}

impl Page for IndexPage {
    fn render(&mut self, siv: &mut CursiveRunnable) {
        let menu = siv.menubar();
        menu.add_subtree(
            "Peel",
            Tree::new()
                .leaf("密钥配置", |s| {})
                .leaf("软件设置", |s| {})
                .leaf("服务器配置", |s| {})
                .delimiter()
                .leaf("断开连接并退出", |s| s.quit()),
        );
        menu.add_subtree(
            "帮助",
            Tree::new()
                .leaf("关于", |s| {})
                .leaf("使用帮助", |s| {})
                .leaf("反馈", |s| {}),
        );
        let filler = "░".repeat(80);
        let head_list = vec!["░█▀█░█▀▀░█▀▀░█░░", "░█▀▀░█▀▀░█▀▀░█░░", "░▀░░░▀▀▀░▀▀▀░▀▀▀"];
        let mut header = LinearLayout::vertical();
        for line in head_list {
            header = header.child(
                LinearLayout::horizontal()
                    .child(TextView::new(filler.clone()).full_width())
                    .child(TextView::new(line).center())
                    .child(TextView::new(filler.clone()).full_width()),
            );
        }
        let panel = Panel::new(TextView::new("当前无连接"));
        let content = LinearLayout::vertical()
            .child(header)
            .child(panel.full_screen());
        let monitor = LinearLayout::horizontal()
            .child(TextView::new("127.0.0.1:8080"))
            .child(TextView::new("运行中"));
        let layout = LinearLayout::vertical()
            .child(content.full_screen())
            .child(monitor);
        siv.add_layer(layout);
        siv.set_autohide_menu(false);
        // siv.select_menubar();
    }
}
