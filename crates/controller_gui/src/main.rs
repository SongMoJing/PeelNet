slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    // let ui_handle = ui.as_weak();
    // ui.on_request_greet(move |name| {
    //     let ui = ui_handle.unwrap();
    //     if name.is_empty() {
    //         ui.set_greeting("名字不能为空哦！".into());
    //     } else {
    //         let result = format!("你好, {}! 这是一个极致轻量的窗口。", name);
    //         ui.set_greeting(result.into());
    //     }
    // });

    ui.run()
}