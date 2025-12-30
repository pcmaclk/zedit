//! 国际化(i18n)模块
//! 提供多语言支持

use std::collections::HashMap;
use std::sync::OnceLock;

/// 语言类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    ChineseSimplified, // 简体中文
    English,           // 英文
                       // 可以继续添加其他语言
}

/// 翻译键
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslationKey {
    // 菜单栏
    MenuFile,
    MenuEdit,
    MenuView,

    // 文件菜单项
    FileOpen,
    FileSave,
    FileExit,

    // 编辑菜单项
    EditUndo,
    EditRedo,

    // 视图菜单项
    ViewTheme,

    // 工具栏按钮
    ToolbarOpen,
    ToolbarSave,
    ToolbarUndo,
    ToolbarRedo,
    ToolbarFind,

    // 标签页
    TabUntitled,
    TabNew,

    // 状态栏
    StatusLine,
    StatusColumn,
    StatusEncoding,
    StatusFileType,
    StatusReady,

    // 对话框
    DialogOpenTitle,
    DialogSaveTitle,
    DialogFilterAll,
    DialogFilterText,

    // 通用
    Yes,
    No,
    OK,
    Cancel,
}

/// 国际化管理器
pub struct I18n {
    current_lang: Language,
    translations: HashMap<Language, HashMap<TranslationKey, String>>,
}

impl I18n {
    pub fn new() -> Self {
        let mut translations = HashMap::new();

        // 简体中文翻译
        let mut zh_cn = HashMap::new();
        zh_cn.insert(TranslationKey::MenuFile, "文件".to_string());
        zh_cn.insert(TranslationKey::MenuEdit, "编辑".to_string());
        zh_cn.insert(TranslationKey::MenuView, "视图".to_string());
        zh_cn.insert(TranslationKey::FileOpen, "打开".to_string());
        zh_cn.insert(TranslationKey::FileSave, "保存".to_string());
        zh_cn.insert(TranslationKey::FileExit, "退出".to_string());
        zh_cn.insert(TranslationKey::EditUndo, "撤销".to_string());
        zh_cn.insert(TranslationKey::EditRedo, "重做".to_string());
        zh_cn.insert(TranslationKey::ViewTheme, "主题切换".to_string());
        zh_cn.insert(TranslationKey::ToolbarOpen, "📂 打开".to_string());
        zh_cn.insert(TranslationKey::ToolbarSave, "💾 保存".to_string());
        zh_cn.insert(TranslationKey::ToolbarUndo, "↶ 撤销".to_string());
        zh_cn.insert(TranslationKey::ToolbarRedo, "↷ 重做".to_string());
        zh_cn.insert(TranslationKey::ToolbarFind, "🔍 查找".to_string());
        zh_cn.insert(TranslationKey::TabUntitled, "[未命名]".to_string());
        zh_cn.insert(TranslationKey::TabNew, "+".to_string());
        zh_cn.insert(TranslationKey::StatusLine, "行".to_string());
        zh_cn.insert(TranslationKey::StatusColumn, "列".to_string());
        zh_cn.insert(TranslationKey::StatusEncoding, "UTF-8".to_string());
        zh_cn.insert(TranslationKey::StatusFileType, "纯文本".to_string());
        zh_cn.insert(TranslationKey::StatusReady, "就绪".to_string());
        zh_cn.insert(TranslationKey::DialogOpenTitle, "打开文件".to_string());
        zh_cn.insert(TranslationKey::DialogSaveTitle, "保存文件".to_string());
        zh_cn.insert(TranslationKey::DialogFilterAll, "所有文件".to_string());
        zh_cn.insert(TranslationKey::DialogFilterText, "文本文件".to_string());
        zh_cn.insert(TranslationKey::Yes, "是".to_string());
        zh_cn.insert(TranslationKey::No, "否".to_string());
        zh_cn.insert(TranslationKey::OK, "确定".to_string());
        zh_cn.insert(TranslationKey::Cancel, "取消".to_string());

        // 英文翻译
        let mut en = HashMap::new();
        en.insert(TranslationKey::MenuFile, "File".to_string());
        en.insert(TranslationKey::MenuEdit, "Edit".to_string());
        en.insert(TranslationKey::MenuView, "View".to_string());
        en.insert(TranslationKey::FileOpen, "Open".to_string());
        en.insert(TranslationKey::FileSave, "Save".to_string());
        en.insert(TranslationKey::FileExit, "Exit".to_string());
        en.insert(TranslationKey::EditUndo, "Undo".to_string());
        en.insert(TranslationKey::EditRedo, "Redo".to_string());
        en.insert(TranslationKey::ViewTheme, "Switch Theme".to_string());
        en.insert(TranslationKey::ToolbarOpen, "📂 Open".to_string());
        en.insert(TranslationKey::ToolbarSave, "💾 Save".to_string());
        en.insert(TranslationKey::ToolbarUndo, "↶ Undo".to_string());
        en.insert(TranslationKey::ToolbarRedo, "↷ Redo".to_string());
        en.insert(TranslationKey::ToolbarFind, "🔍 Find".to_string());
        en.insert(TranslationKey::TabUntitled, "[Untitled]".to_string());
        en.insert(TranslationKey::TabNew, "+".to_string());
        en.insert(TranslationKey::StatusLine, "Ln".to_string());
        en.insert(TranslationKey::StatusColumn, "Col".to_string());
        en.insert(TranslationKey::StatusEncoding, "UTF-8".to_string());
        en.insert(TranslationKey::StatusFileType, "Plain Text".to_string());
        en.insert(TranslationKey::StatusReady, "Ready".to_string());
        en.insert(TranslationKey::DialogOpenTitle, "Open File".to_string());
        en.insert(TranslationKey::DialogSaveTitle, "Save File".to_string());
        en.insert(TranslationKey::DialogFilterAll, "All Files".to_string());
        en.insert(TranslationKey::DialogFilterText, "Text Files".to_string());
        en.insert(TranslationKey::Yes, "Yes".to_string());
        en.insert(TranslationKey::No, "No".to_string());
        en.insert(TranslationKey::OK, "OK".to_string());
        en.insert(TranslationKey::Cancel, "Cancel".to_string());

        translations.insert(Language::ChineseSimplified, zh_cn);
        translations.insert(Language::English, en);

        Self {
            current_lang: Language::ChineseSimplified,
            translations,
        }
    }

    /// 获取当前语言
    pub fn get_current_language(&self) -> Language {
        self.current_lang
    }

    /// 设置当前语言
    pub fn set_current_language(&mut self, lang: Language) {
        self.current_lang = lang;
    }

    /// 翻译一个键
    pub fn translate(&self, key: TranslationKey) -> String {
        self.translations
            .get(&self.current_lang)
            .and_then(|lang_translations| lang_translations.get(&key))
            .cloned()
            .unwrap_or_else(|| format!("{:?}", key))
    }

    /// 翻译多个键并连接
    pub fn translate_multi(&self, keys: &[TranslationKey]) -> String {
        keys.iter()
            .map(|&key| self.translate(key))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// 全局I18n实例
static I18N: OnceLock<std::sync::Mutex<I18n>> = OnceLock::new();

/// 获取全局I18n实例
pub fn get_i18n() -> &'static std::sync::Mutex<I18n> {
    I18N.get_or_init(|| std::sync::Mutex::new(I18n::new()))
}

/// 快速翻译函数
pub fn t(key: TranslationKey) -> String {
    let i18n = get_i18n().lock().unwrap();
    i18n.translate(key)
}

/// 设置语言
pub fn set_language(lang: Language) {
    let mut i18n = get_i18n().lock().unwrap();
    i18n.set_current_language(lang);
}

/// 获取当前语言
pub fn get_language() -> Language {
    let i18n = get_i18n().lock().unwrap();
    i18n.get_current_language()
}
