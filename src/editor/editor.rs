use crate::document::Document;

pub struct Editor {
    document: Document,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            document: Document::empty(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::editor::view::editor_view(ui, &self.document);
        });
    }
}
