use cursive::views::{Button, Dialog, LinearLayout, Panel, SelectView, StackView, TextView};
use cursive_tabs::TabPanel;
use cursive::view::Resizable;
use cursive::align::Align;
use cursive::CursiveRunnable;
use cursive::menu::Tree;
use cursive::traits::Nameable;
use crate::pages::Page;

pub struct IndexPage {
}

impl IndexPage {
	pub fn new() -> IndexPage {
		IndexPage {}
	}
}

impl Page for IndexPage {
	fn render(&mut self, siv: &mut CursiveRunnable) {
		// 1. 定义菜单树结构 (这是最标准、最 IDE 的实现方式)
		let mut menu = siv.menubar();
		menu.add_subtree("文件(F)", Tree::new()
			.leaf("新建", |s| { /* 执行新建逻辑 */ })
			.leaf("打开", |s| { /* 执行打开逻辑 */ })
			.delimiter()
			.leaf("退出", |s| s.quit()));

		// 2. 标签栏 (紧贴菜单栏下方)
		let mut tab_bar = LinearLayout::horizontal();
		tab_bar.add_child(TextView::new(" [main.rs] "));
		tab_bar.add_child(TextView::new(" [index.rs] "));

		// 3. 内容区
		let content = Panel::new(TextView::new("编辑器内容区域").with_name("editor"));

		// 4. 总布局
		let layout = LinearLayout::vertical()
			.child(Panel::new(tab_bar))
			.child(content.full_screen());

		siv.add_layer(layout);

		// 5. 必须开启菜单栏显示
		siv.set_autohide_menu(false);
		siv.select_menubar();
	}
}


