mod app;
mod document;
mod editor;
mod gui;
mod io;
mod plugin;

use app::App;
use gui::MainWindow;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "REditor",
        options,
        Box::new(|cc| {
            configure_fonts(&cc.egui_ctx);

            let app = Rc::new(RefCell::new(App::new()));
            let main_window = MainWindow::new(app);
            Box::new(main_window)
        }),
    )
    .expect("failed to start eframe");
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 1. 尝试加载系统字体文件（不需要 font-kit）
    let font_paths = [
        // Windows
        r"C:\Windows\Fonts\msyh.ttc",   // 微软雅黑
        r"C:\Windows\Fonts\msyh.ttf",   // 微软雅黑
        r"C:\Windows\Fonts\simsun.ttc", // 宋体
        // macOS
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/STHeiti Light.ttc",
        // Linux
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    ];

    for path in &font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            println!("成功加载字体: {}", path);
            fonts.font_data.insert(
                "ChineseFont".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            break;
        }
    }

    // 2. 如果系统字体都没找到，使用内置的备用字体
    if !fonts.font_data.contains_key("ChineseFont") {
        // 这里可以嵌入一个小型字体，或者使用默认字体
        // 对于简单应用，egui 默认字体可能已经能显示基本中文
        println!("未找到系统字体，使用默认字体");
    } else {
        // 设置字体优先级
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "ChineseFont".to_owned());
    }

    ctx.set_fonts(fonts);
}
