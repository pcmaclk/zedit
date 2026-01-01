use ropey::Rope;

pub struct Buffer {
    rope: Rope,
}

impl Buffer {
    pub fn empty() -> Self {
        let text = "Hello, Rust editor MVP!\n\n\
这是一个最小可运行原型。\n\
支持 Ropey 显示。\n\n\
下一步：虚拟行 + 高亮。";

        Self {
            rope: Rope::from_str(text),
        }
    }

    pub fn from_content(content: &str) -> Self {
        Self {
            rope: Rope::from_str(content),
        }
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line(&self, index: usize) -> Option<&str> {
        if index < self.rope.len_lines() {
            self.rope.line(index).as_str()
        } else {
            None
        }
    }

    pub fn get_content(&self) -> String {
        self.rope.to_string()
    }

    pub fn set_content(&mut self, content: &str) {
        self.rope = Rope::from_str(content);
    }
}
