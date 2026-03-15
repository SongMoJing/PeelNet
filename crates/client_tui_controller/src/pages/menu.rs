use cursive::event::Key;
use cursive::menu::Tree;
use cursive::Cursive;

pub fn set_menu(siv: &mut Cursive) {
    siv.menubar()
    .add_subtree(
        "(_P) Peel",
        Tree::new()
            .leaf("密钥配置", |s| {})
            .leaf("软件设置", |s| {})
            .leaf("服务器配置", |s| {})
            .delimiter()
            .leaf("断开连接并退出", |s| s.quit()),
    )
    .add_subtree(
        "(_H) 帮助",
        Tree::new()
            .leaf("关于", |s| {})
            .leaf("使用帮助", |s| {})
            .leaf("反馈", |s| {}),
    );
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
}
