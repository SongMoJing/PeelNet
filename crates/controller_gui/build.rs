extern crate winres;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        // 处理图标
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/icon.ico");
        if let Err(e) = res.compile() {
            eprintln!("图标错误: {}", e);
            std::process::exit(1);
        }
        // 只有 Release 模式且使用 MSVC 时禁用控制台
        let profile = std::env::var("PROFILE").unwrap();
        if profile == "release" && target.contains("msvc") {
            println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
            println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
        }
    }
}
