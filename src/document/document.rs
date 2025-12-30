use super::buffer::Buffer;

pub struct Document {
    buffer: Buffer,
}

impl Document {
    pub fn empty() -> Self {
        Self {
            buffer: Buffer::empty(),
        }
    }

    pub fn line_count(&self) -> usize {
        self.buffer.line_count()
    }

    pub fn line(&self, index: usize) -> Option<&str> {
        self.buffer.line(index)
    }
}
