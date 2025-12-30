use crate::document::Document;
use crate::gui::editor_view;

pub struct Editor {
    pub document: Document,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            document: Document::empty(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            editor_view(ui, &self.document);
        });
    }
}
