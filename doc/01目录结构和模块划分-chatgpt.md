**最终推荐版（V1.0 冻结）**。

---

# 一、整体分层总览（先给你“地图”）

```
core        → 纯数据结构（最底层）
editor      → 编辑器状态 & 编辑逻辑
features    → 编辑算法（无状态）
search      → 查询 / 替换
syntax      → 语法分析
theme       → 主题 & 样式
render      → 文档 → 像素
gui         → egui 组件
input       → 键鼠 → Action
io          → 文件系统
encoding    → 编码处理
i18n        → 国际化
app         → 生命周期 & 调度（最顶层）
```

**唯一可变核心：`editor`**

---

# 二、完整目录结构 + 逐文件说明

---

## 根目录

```
editor/
├─ Cargo.toml            # Rust 项目依赖与特性开关
├─ build.rs              # 构建期资源嵌入（主题 / 语法 / 图标）
├─ .editorconfig         # 统一代码风格
```

---

## src/main.rs

```
src/
└─ main.rs
```

**职责：**

* 程序入口
* 初始化 app
* 启动 GUI runtime（egui / winit）

❌ 不包含业务逻辑

---

## core/ —— 最底层（禁止依赖任何模块）

```
core/
├─ mod.rs                # core 模块导出
├─ position.rs           # 行/列/字节偏移等基础位置类型
├─ buffer.rs             # 文本缓冲（Rope / GapBuffer）
├─ cursor.rs             # 光标数据结构
└─ selection.rs          # 选区数据结构
```

**原则：**

* 无 IO
* 无 GUI
* 无主题
* 可独立做成 crate

---

## editor/ —— 编辑器核心（唯一可变状态）

```
editor/
├─ mod.rs
├─ editor.rs             # Editor 主体：聚合 document / cursor / history
├─ document.rs           # 文档模型（buffer + metadata）
├─ metadata.rs           # 文件路径 / 编码 / 换行符等
├─ flags.rs              # dirty / readonly / large_file
├─ history.rs            # Undo / Redo 栈
├─ operation.rs          # 原子操作定义（Insert/Delete）
├─ edit_ops.rs           # 组合编辑操作
├─ indent.rs             # 自动缩进 / tab-space
└─ line_ending.rs        # LF / CRLF 管理
```

**规则：**

* ✅ 可以依赖 core
* ❌ 不能依赖 render / gui
* ❌ 不知道屏幕、不知道像素

---

## features/ —— 编辑算法（纯函数）

```
features/
├─ mod.rs
├─ bracket_match.rs      # 括号匹配 / 高亮
├─ goto.rs               # 跳转到行 / 匹配括号
└─ wrap.rs               # 软换行算法
```

**特点：**

* 输入：editor / buffer
* 输出：结果（位置、范围）
* ❌ 不修改 editor
* ❌ 不绘制

---

## search/ —— 查询系统

```
search/
├─ mod.rs
├─ searcher.rs           # 查找（普通 / 正则）
└─ replacer.rs           # 替换（单个 / 全部）
```

---

## syntax/ —— 语法分析

```
syntax/
├─ mod.rs
├─ token.rs              # 语法 token 定义
├─ parser.rs             # 语法解析接口
└─ syntect_adapter.rs    # Syntect 集成
```

**职责：**

* 只产出 token / scope
* 不决定颜色

---

## theme/ —— 主题与样式（跨层资源）

```
theme/
├─ mod.rs
├─ theme.rs              # Theme 数据结构
├─ colors.rs             # 颜色定义
├─ loader.rs             # TOML / JSON 加载
└─ builtin.rs            # 内置主题
```

---

## render/ —— 渲染层（怎么画）

```
render/
├─ mod.rs
├─ view_model.rs         # Editor → ViewModel（核心桥梁）
├─ style_resolver.rs     # token + theme → style
├─ layout.rs             # 行布局 / 字符位置
├─ text_run.rs           # 带样式文本段
├─ viewport.rs           # 可视区域 / 虚拟滚动
├─ gutter.rs             # 行号渲染 ⭐
├─ cursor_renderer.rs    # 光标绘制
└─ selection_renderer.rs # 选区绘制
```

**render 的输入：**

* EditorViewModel
* Theme

**render 的输出：**

* egui 绘制调用

---

## gui/ —— GUI 组件（egui）

```
gui/
├─ mod.rs
├─ main_window.rs        # 主窗口布局
├─ menu.rs               # 菜单栏
├─ dialogs.rs            # 打开 / 保存 / 查找对话框
├─ statusbar.rs          # 状态栏
├─ context_menu.rs       # 右键菜单
│
└─ editor_view/          # 编辑器视图 ⭐
   ├─ mod.rs
   ├─ editor_view.rs     # 编辑器视图主入口
   ├─ text_layer.rs      # 文本层
   ├─ selection_layer.rs # 选区层
   ├─ cursor_layer.rs    # 光标层
   └─ input_mapper.rs    # 鼠标 → Action
```

**editor_view 是 GUI 中唯一“理解编辑器视觉语义”的模块**

---

## input/ —— 输入系统

```
input/
├─ mod.rs
├─ keyboard.rs           # 键盘事件处理
├─ mouse.rs              # 鼠标事件处理
└─ keybindings.rs        # 快捷键映射
```

**输出统一为：**

```
Action
```

---

## app/ —— 应用层（最顶层）

```
app/
├─ mod.rs
├─ app.rs                # 应用生命周期
├─ session.rs            # 会话恢复
├─ config.rs             # 配置管理
└─ command/
   ├─ mod.rs
   ├─ action.rs          # 用户动作（唯一入口）
   ├─ app_command.rs     # 应用级命令
   ├─ editor_command.rs  # 编辑命令
   └─ dispatcher.rs      # Action → 执行
```

**所有修改 editor 的行为都必须经过 dispatcher**

---

## io/ —— 文件系统

```
io/
├─ mod.rs
├─ file.rs               # 打开 / 保存
├─ mmap.rs               # 大文件支持
└─ watcher.rs            # 文件变更监听
```

---

## encoding/ —— 编码处理

```
encoding/
├─ mod.rs
├─ detect.rs             # 编码检测
└─ convert.rs            # 编码转换
```

---

## i18n/ —— 国际化

```
i18n/
├─ mod.rs
├─ language.rs           # 当前语言
└─ strings.rs            # 翻译字符串
```

---

## tests / benches / docs

```
tests/                   # 功能测试
benches/                 # 性能测试
docs/                    # 架构文档
```

---

# 三、模块依赖铁律（总结）

✔ **允许方向（从下往上）**

```
core → editor → render → gui → app
```

❌ **禁止方向**

* core ❌ editor
* editor ❌ render
* render ❌ editor
* gui ❌ editor（只能发 Action）

---

[模块依赖铁律]

core
  - 不得 import 任何模块

theme / encoding / i18n
  - 只允许 import core

syntax / search
  - 允许 import core / theme
  - 禁止 import editor / render / app

features
  - 允许 import core / syntax / search
  - 禁止 import editor / render / gui / app / io

editor
  - 允许 import core / syntax / search / features
  - 禁止 import io / gui / render

render
  - 允许 import core / theme
  - 禁止 import editor
  - 只能接收 ViewModel（不可变）

input / gui
  - 不得包含业务逻辑
  - 只能产生 Action

app
  - 唯一允许 import io 的模块
  
---