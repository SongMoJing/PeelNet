use crate::model::config::Config;
use crate::model::saved::Saved;
use crate::model::state::State;
use cursive::event::Key;
use cursive::menu::Tree;
use cursive::style::{Color, ColorStyle, PaletteColor};
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, NamedView, Panel, ResizedView, TextView};
use cursive::Cursive;
use std::rc::Rc;

pub fn set_menu(siv: &mut Cursive, config: Rc<Config>, state: Rc<State>, saved: Rc<Saved>) {
    siv.menubar()
        .add_subtree(
            "Peel",
            Tree::new()
                .leaf("新建连接", move |s| {
                    let mut dialog = Dialog::new()
                        .content(
                            LinearLayout::vertical()
                                .child(
                                    LinearLayout::horizontal()
                                        .child(TextView::new("请输入服务器名称"))
                                        .child(
                                            EditView::new()
                                                .with_name("dialog:input:name")
                                                .full_width(),
                                        ),
                                )
                                .child(
                                    LinearLayout::horizontal()
                                        .child(TextView::new("请输入服务器端口"))
                                        .child(
                                            EditView::new()
                                                .with_name("dialog:input:port")
                                                .full_width(),
                                        ),
                                )
                                .child(
                                    LinearLayout::horizontal()
                                        .child(TextView::new("请输入服务器用户名"))
                                        .child(
                                            EditView::new()
                                                .with_name("dialog:input:username")
                                                .full_width(),
                                        ),
                                ),
                        )
                        .button("取消", |s| {
                            s.pop_layer();
                        })
                        .button("确定", |s| {
                            let name = s
                                .call_on_name("dialog:input:name", |view: &mut EditView| {
                                    view.get_content()
                                });
                            if name.is_none() {
                                return;
                            }
                            let port = s
                                .call_on_name("dialog:input:port", |view: &mut EditView| {
                                    view.get_content()
                                });
                            if port.is_none() {
                                return;
                            }
                            let username = s
                                .call_on_name("dialog:input:username", |view: &mut EditView| {
                                    view.get_content()
                                });
                            if username.is_none() {
                                return;
                            }
                            s.pop_layer();
                        });
                    dialog.set_title("新建连接");
                    s.add_layer(
                        dialog
                            .min_width(40)
                            .min_height(10)
                            .max_width(70)
                            .max_height(20),
                    );
                })
                .leaf("密钥配置", |s| {
                    // siv.call_on_name("header", |view: &mut TextView| {
                    //     view.set_style(ColorStyle::new(
                    //         PaletteColor::HighlightText,
                    //         Color::TerminalDefault,
                    //     ))
                    // });
                    s.call_on_name(
                        "board",
                        |view: &mut Panel<NamedView<ResizedView<LinearLayout>>>| {
                            view.set_title("密钥配置")
                        },
                    );
                    s.call_on_name("content", |view: &mut LinearLayout| {
                        view.clear();
                        view.add_child(TextView::new("这是新替换的视图"));
                    });
                })
                .leaf("软件设置", |s| {})
                .leaf("服务器配置", |s| {})
                .delimiter()
                .leaf("断开连接并退出", |s| s.quit()),
        )
        .add_subtree(
            "帮助",
            Tree::new()
                .leaf("关于", |s| {})
                .leaf("使用帮助", |s| {})
                .leaf("反馈", |s| {}),
        );
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
}
