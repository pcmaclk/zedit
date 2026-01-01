好，我们继续 **把「语法高亮完整接入 EditorViewModel 生成流程」**。
这一步做完，你的架构会达到 **Notepad4 / VS Code 同一层级的渲染管线形态**。

> 🎯 目标
> **Editor → Snapshot →（Syntax）→（Theme）→ Styled EditorViewModel → Render**

---

# 一、我们现在所处的位置（快速对齐）

你已经有：

* `EditorSnapshot { text, cursor, selection }`
* `ViewModelBuilder` 负责生成 `EditorViewModel`
* `render` 只吃 ViewModel

**缺的只有一件事**：
👉 **ViewModel 里没有“样式化的文本”**

---

# 二、引入「StyledTextRun」模型（关键一步）

## 设计原则（非常重要）

* ViewModel **不暴露 Token**
* ViewModel **不关心 Syntect**
* ViewModel **只包含最终可渲染的数据**

---

## render/view_model.rs（升级）

```rust
use crate::core::{
    cursor::Cursor,
    selection::Selection,
};

#[derive(Clone)]
pub struct EditorViewModel {
    pub lines: Vec<ViewLine>,
    pub cursor: Cursor,
    pub selection: Selection,
    pub show_line_numbers: bool,
    pub first_visible_line: usize,
}

#[derive(Clone)]
pub struct ViewLine {
    pub line_index: usize,
    pub runs: Vec<StyledTextRun>,
}

#[derive(Clone)]
pub struct StyledTextRun {
    pub text: String,
    pub style: TextStyle,
}

#[derive(Clone, Copy)]
pub struct TextStyle {
    pub fg: (u8, u8, u8),
    pub bold: bool,
    pub italic: bool,
}
```

✅ **渲染层此时已经不需要 syntax / token**

---

# 三、Syntax 层的最小可用接口

## syntax/parser.rs（回顾 + 明确）

```rust
use super::token::Token;

pub trait SyntaxParser {
    fn parse_line(&self, line: &str) -> Vec<Token>;
}
```

> ⚠️ **按“行”解析是刻意设计**
>
> * ViewModel 是按行生成
> * 渐进渲染 / 大文件更友好

---

## syntax/token.rs（补充位置信息）

```rust
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Keyword,
    String,
    Comment,
    Plain,
}
```

---

# 四、Theme → Style 映射（StyleResolver）

## 新模块

```
src/render/style_resolver.rs
```

---

## render/style_resolver.rs

```rust
use crate::{
    syntax::token::{Token, TokenKind},
    render::view_model::{StyledTextRun, TextStyle},
    theme::theme::Theme,
};

pub struct StyleResolver<'a> {
    theme: &'a Theme,
}

impl<'a> StyleResolver<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    pub fn resolve_tokens(&self, tokens: Vec<Token>) -> Vec<StyledTextRun> {
        tokens
            .into_iter()
            .map(|token| {
                let fg = self
                    .theme
                    .syntax
                    .get(&token.kind)
                    .copied()
                    .unwrap_or(self.theme.foreground);

                StyledTextRun {
                    text: token.text,
                    style: TextStyle {
                        fg,
                        bold: token.kind == TokenKind::Keyword,
                        italic: token.kind == TokenKind::Comment,
                    },
                }
            })
            .collect()
    }
}
```

📌 **职责非常清晰**

| 模块             | 做什么                |
| -------------- | ------------------ |
| syntax         | text → token       |
| theme          | token → color      |
| style_resolver | token → styled run |

---

# 五、ViewModelBuilder：接入 Syntax + Theme

这是**最核心的升级**。

---

## render/view_model_builder.rs（完整版）

```rust
use crate::{
    editor::editor::EditorSnapshot,
    render::view_model::{EditorViewModel, ViewLine},
    render::style_resolver::StyleResolver,
    syntax::parser::SyntaxParser,
    theme::theme::Theme,
};

pub struct ViewModelBuilder<'a> {
    pub viewport_lines: usize,
    pub parser: &'a dyn SyntaxParser,
    pub theme: &'a Theme,
}

impl<'a> ViewModelBuilder<'a> {
    pub fn new(
        viewport_lines: usize,
        parser: &'a dyn SyntaxParser,
        theme: &'a Theme,
    ) -> Self {
        Self {
            viewport_lines,
            parser,
            theme,
        }
    }

    pub fn build(
        &self,
        snapshot: &EditorSnapshot,
        first_visible_line: usize,
    ) -> EditorViewModel {
        let resolver = StyleResolver::new(self.theme);

        let all_lines: Vec<&str> = snapshot.text.lines().collect();

        let lines = all_lines
            .iter()
            .enumerate()
            .skip(first_visible_line)
            .take(self.viewport_lines)
            .map(|(i, line)| {
                let tokens = self.parser.parse_line(line);
                let runs = resolver.resolve_tokens(tokens);

                ViewLine {
                    line_index: i,
                    runs,
                }
            })
            .collect();

        EditorViewModel {
            lines,
            cursor: snapshot.cursor.clone(),
            selection: snapshot.selection.clone(),
            show_line_numbers: true,
            first_visible_line,
        }
    }
}
```

---

# 六、Renderer 如何消费 StyledTextRun

## render/renderer.rs（升级）

```rust
use crate::render::view_model::EditorViewModel;

pub struct Renderer;

impl Renderer {
    pub fn render(&self, vm: &EditorViewModel) {
        for line in &vm.lines {
            for run in &line.runs {
                self.draw_run(run);
            }
        }
    }

    fn draw_run(&self, run: &crate::render::view_model::StyledTextRun) {
        // egui / wgpu / skia 绘制
        // run.text
        // run.style.fg
        // run.style.bold / italic
    }
}
```

🎉 **render 层至此完全不知道 syntax / editor / theme**

---

# 七、完整依赖方向（现在是“教科书级”）

```
editor ── snapshot
   ↓
ViewModelBuilder
   ↓
syntax → token
   ↓
theme → style
   ↓
EditorViewModel
   ↓
render / gui
```

🚫 禁止反向依赖：

* render ❌ syntax
* render ❌ editor
* syntax ❌ theme
* theme ❌ editor

---

# 八、你现在已经具备的能力

你已经可以：

* ✔ 支持 **任意语法高亮引擎**
* ✔ 支持 **热切换主题**
* ✔ 支持 **超大文件（可见行渲染）**
* ✔ 未来支持 **Tree-sitter / Syntect / LSP**

---

# 九、下一步（强烈推荐顺序）

你已经站在一个**关键分叉点**，下面是**最正确的继续路线**：

### 🔜 Step 1（最自然）

👉 **加入 layout（字符 → 像素）**

* glyph width
* tab
* wrap
* cursor x/y

### 🔜 Step 2

👉 **cursor / selection 命中 StyledTextRun**

* 光标落在哪个 run
* 选区如何跨 run

### 🔜 Step 3

👉 **增量 ViewModel（性能关键）**

* buffer diff
* 行级 dirty 标记

---

如果你愿意，我可以下一步**直接把 layout + cursor 像素定位完整设计并写骨架代码**。
这一步做完，你的编辑器就正式迈入「专业编辑器」范畴。
