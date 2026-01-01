use crate::app::App;
use crate::gui::editor_view;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainWindow {
    app: Rc<RefCell<App>>,
}

impl MainWindow {
    pub fn new(app: Rc<RefCell<App>>) -> Self {
        Self { app }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        // 菜单栏
        self.draw_menu(ctx);

        // 工具栏
        self.draw_toolbar(ctx);

        // 标签页
        self.draw_tabs(ctx);

        // 编辑区
        self.draw_editor(ctx);

        // 状态栏
        self.draw_status(ctx);
    }

    fn draw_menu(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("打开").clicked() {
                        let mut app = self.app.borrow_mut();
                        if app.editor.open_file() {
                            println!("文件打开成功");
                        }
                        ui.close_menu();
                    }
                    if ui.button("保存").clicked() {
                        let mut app = self.app.borrow_mut();
                        if app.editor.save_file() {
                            println!("文件保存成功");
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("退出").clicked() {
                        // TODO: 退出程序
                        ui.close_menu();
                    }
                });

                ui.menu_button("编辑", |ui| {
                    if ui.button("撤销").clicked() {
                        // TODO: 撤销
                        ui.close_menu();
                    }
                    if ui.button("重做").clicked() {
                        // TODO: 重做
                        ui.close_menu();
                    }
                });

                ui.menu_button("视图", |ui| {
                    if ui.button("主题切换").clicked() {
                        // TODO: 主题切换
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn draw_toolbar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("📂 打开").clicked() {
                    let mut app = self.app.borrow_mut();
                    app.editor.open_file();
                }
                if ui.button("💾 保存").clicked() {
                    let mut app = self.app.borrow_mut();
                    app.editor.save_file();
                }
                ui.separator();
                if ui.button("↶ 撤销").clicked() {
                    // TODO: 撤销
                }
                if ui.button("↷ 重做").clicked() {
                    // TODO: 重做
                }
                ui.separator();
                if ui.button("🔍 查找").clicked() {
                    // TODO: 查找
                }
            });
        });
    }

    fn draw_tabs(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let app = self.app.borrow();
                let file_name = app.editor.document.get_file_name();
                let dirty_marker = if app.editor.document.is_dirty() { " *" } else { "" };
                ui.label(format!("{}{}", file_name, dirty_marker));
                
                if ui.button("+").clicked() {
                    // TODO: 新建文档
                }
            });
        });
    }

    fn draw_editor(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let app = self.app.borrow();
            // 使用 editor_view 渲染编辑区
            editor_view::editor_view(ui, &app.editor.document);
        });
    }

    fn draw_status(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 行号和列号
                ui.label("行:1 列:1");
                ui.separator();

                // 编码
                ui.label("UTF-8");
                ui.separator();

                // 文件类型
                ui.label("Plain Text");
                ui.separator();

                // 状态
                let app = self.app.borrow();
                if app.editor.document.is_dirty() {
                    ui.label("已修改");
                } else {
                    ui.label("就绪");
                }
            });
        });
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}
