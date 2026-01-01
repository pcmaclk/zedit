use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

/// 打开文件选择对话框并返回选中的文件路径
pub fn open_file_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Text Files", &["txt", "rs", "toml", "md", "json", "yaml", "yml", "xml", "html", "css", "js", "ts", "py", "java", "c", "cpp", "h", "hpp"])
        .add_filter("All Files", &["*"])
        .pick_file()
}

/// 读取文件内容
pub fn read_file(path: &PathBuf) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// 保存文件内容
pub fn save_file(path: &PathBuf, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}

/// 打开文件并返回内容和路径
pub fn open_file() -> Option<(String, PathBuf)> {
    if let Some(path) = open_file_dialog() {
        match read_file(&path) {
            Ok(content) => Some((content, path)),
            Err(e) => {
                eprintln!("读取文件失败: {}", e);
                None
            }
        }
    } else {
        None
    }
}