---

# 一、EditorViewModel 在架构中的精确定位

### 它是什么？

**EditorViewModel 是“只读、扁平、一次性”的渲染输入数据**

* 不可变
* 不持有 Editor 引用
* 不包含 Rope / Cursor / EditorState
* **完全可以脱离 editor 测试**

### 它不是什么？

* ❌ 不做布局
* ❌ 不做语法解析
* ❌ 不保存编辑状态
* ❌ 不处理输入

---

# 二、完整数据流（非常关键）

```
Editor（真实状态）
  │
  │ snapshot()
  ▼
EditorSnapshot（内部快照）
  │
  │ ViewModelBuilder
  ▼
EditorViewModel（渲染 DTO）
  │
  ▼
render / gui
```

> ⚠️ **Editor 不知道 ViewModel 存在**
>
> ViewModelBuilder 才是“胶水层”

---

# 三、EditorSnapshot（编辑器内部只读快照）

## editor/editor.rs（补全）

```rust
use crate::core::{
    buffer::Buffer,
    cursor::Cursor,
    selection::Selection,
};

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    selection: Selection,
}

impl Editor {
    pub fn snapshot(&self) -> EditorSnapshot {
        EditorSnapshot {
            text: self.buffer.text(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        }
    }
}

/// ⚠️ Editor 内部结构，不暴露给 render
pub struct EditorSnapshot {
    pub text: String,
    pub cursor: Cursor,
    pub selection: Selection,
}
```

---

# 四、EditorViewModel 定义（render 层）

## render/view_model.rs

```rust
use crate::core::{
    cursor::Cursor,
    selection::Selection,
};

pub struct EditorViewModel {
    pub lines: Vec<ViewLine>,
    pub cursor: Cursor,
    pub selection: Selection,

    // UI flags
    pub show_line_numbers: bool,
    pub first_visible_line: usize,
}

pub struct ViewLine {
    pub line_index: usize,
    pub text: String,
}
```

设计说明：

| 字段                   | 说明              |
| -------------------- | --------------- |
| `lines`              | **已经裁剪好的可见行**   |
| `cursor`             | 直接传值，render 不修改 |
| `selection`          | 同上              |
| `first_visible_line` | gutter 用        |
| `show_line_numbers`  | GUI 控制          |

---

# 五、ViewModelBuilder（核心生成逻辑）

> ⚠️ **这是整个架构的“中枢”**

## 新增模块

```
src/
 └─ render/
     ├─ view_model.rs
     └─ view_model_builder.rs  ← 新增
```

---

## render/view_model_builder.rs

```rust
use crate::{
    editor::editor::EditorSnapshot,
    render::view_model::{EditorViewModel, ViewLine},
};

pub struct ViewModelBuilder {
    pub viewport_lines: usize,
}

impl ViewModelBuilder {
    pub fn new(viewport_lines: usize) -> Self {
        Self { viewport_lines }
    }

    pub fn build(
        &self,
        snapshot: &EditorSnapshot,
        first_visible_line: usize,
    ) -> EditorViewModel {
        let all_lines: Vec<&str> = snapshot.text.lines().collect();

        let visible_lines = all_lines
            .iter()
            .enumerate()
            .skip(first_visible_line)
            .take(self.viewport_lines)
            .map(|(i, line)| ViewLine {
                line_index: i,
                text: line.to_string(),
            })
            .collect();

        EditorViewModel {
            lines: visible_lines,
            cursor: snapshot.cursor.clone(),
            selection: snapshot.selection.clone(),
            show_line_numbers: true,
            first_visible_line,
        }
    }
}
```

---

# 六、GUI / EditorView 使用方式（完整闭环）

## gui/editor_view.rs（更新）

```rust
use crate::{
    editor::editor::Editor,
    render::{
        view_model_builder::ViewModelBuilder,
        renderer::Renderer,
        gutter::Gutter,
    },
};

pub struct EditorView {
    renderer: Renderer,
    gutter: Gutter,
    vm_builder: ViewModelBuilder,
}

impl EditorView {
    pub fn new(viewport_lines: usize) -> Self {
        Self {
            renderer: Renderer,
            gutter: Gutter,
            vm_builder: ViewModelBuilder::new(viewport_lines),
        }
    }

    pub fn draw(&self, editor: &Editor, first_visible_line: usize) {
        let snapshot = editor.snapshot();
        let vm = self.vm_builder.build(&snapshot, first_visible_line);

        self.gutter.paint(&vm);
        self.renderer.render(&vm);
    }
}
```

---

# 七、Renderer / Gutter 如何使用 ViewModel

## render/gutter.rs（示例）

```rust
use crate::render::view_model::EditorViewModel;

pub struct Gutter;

impl Gutter {
    pub fn paint(&self, vm: &EditorViewModel) {
        if !vm.show_line_numbers {
            return;
        }

        for line in &vm.lines {
            let number = line.line_index + 1;
            // draw number
        }
    }
}
```

---

## render/renderer.rs

```rust
use crate::render::view_model::EditorViewModel;

pub struct Renderer;

impl Renderer {
    pub fn render(&self, vm: &EditorViewModel) {
        for line in &vm.lines {
            // draw line.text
        }
    }
}
```

---

# 八、为什么这是“正确”的 ViewModel 流程

### ✅ 彻底解耦

* render **永远不碰 Editor**
* Editor **不知道 render 存在**

### ✅ 可测试

```rust
#[test]
fn build_view_model() {
    let snapshot = EditorSnapshot {
        text: "a\nb\nc".into(),
        cursor: dummy_cursor(),
        selection: dummy_selection(),
    };

    let builder = ViewModelBuilder::new(2);
    let vm = builder.build(&snapshot, 1);

    assert_eq!(vm.lines.len(), 2);
    assert_eq!(vm.lines[0].text, "b");
}
```

### ✅ 可扩展（后续你一定会加）

* 语法高亮 → `StyledViewLine`
* 折叠 → `is_collapsed`
* 软换行 → `visual_lines`
* minimap → 重用 ViewModel

---
