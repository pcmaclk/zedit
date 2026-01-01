use super::buffer::Buffer;
use std::path::PathBuf;

pub struct Document {
    buffer: Buffer,
    file_path: Option<PathBuf>,
    is_dirty: bool,
}

impl Document {
    pub fn empty() -> Self {
        Self {
            buffer: Buffer::empty(),
            file_path: None,
            is_dirty: false,
        }
    }

    pub fn from_content(content: &str, path: Option<PathBuf>) -> Self {
        Self {
            buffer: Buffer::from_content(content),
            file_path: path,
            is_dirty: false,
        }
    }

    pub fn line_count(&self) -> usize {
        self.buffer.line_count()
    }

    pub fn line(&self, index: usize) -> Option<&str> {
        self.buffer.line(index)
    }

    pub fn get_content(&self) -> String {
        self.buffer.get_content()
    }

    pub fn set_content(&mut self, content: &str) {
        self.buffer.set_content(content);
        self.is_dirty = true;
    }

    pub fn get_file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path = Some(path);
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.is_dirty = dirty;
    }

    pub fn get_file_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "未命名".to_string())
    }
}
