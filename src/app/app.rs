use crate::editor::Editor;

pub struct App {
    pub editor: Editor,
}

impl App {
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.editor.ui(ctx);
    }
}
