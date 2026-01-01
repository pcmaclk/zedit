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

    pub fn open_file(&mut self) -> bool {
        if let Some((content, path)) = crate::io::file::open_file() {
            self.document = Document::from_content(&content, Some(path));
            true
        } else {
            false
        }
    }

    pub fn save_file(&mut self) -> bool {
        if let Some(path) = self.document.get_file_path() {
            let content = self.document.get_content();
            match crate::io::file::save_file(path, &content) {
                Ok(_) => {
                    self.document.set_dirty(false);
                    true
                }
                Err(e) => {
                    eprintln!("保存文件失败: {}", e);
                    false
                }
            }
        } else {
            // 如果没有文件路径，需要调用另存为
            self.save_file_as()
        }
    }

    pub fn save_file_as(&mut self) -> bool {
        // TODO: 实现另存为对话框
        false
    }
}
