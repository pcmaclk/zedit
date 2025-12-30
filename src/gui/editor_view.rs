use crate::document::Document;

pub fn editor_view(ui: &mut egui::Ui, document: &Document) {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            let line_count = document.line_count();

            for i in 0..line_count {
                if let Some(line) = document.line(i) {
                    ui.label(line);
                }
            }
        });
}
