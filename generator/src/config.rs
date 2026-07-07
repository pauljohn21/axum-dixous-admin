//! 配置文件解析模块
//!
//! YAML 配置文件格式：
//! ```yaml
//! table_name: sys_products       # 数据库表名 (sys_ 前缀)
//! resource: product               # 资源名 (snake_case 单数)
//! module_cn: "产品管理"            # 中文模块名
//! icon: goods                     # Element Plus 图标
//! fields:
//!   - name: name
//!     type: string
//!     nullable: true
//!     comment: "产品名称"
//!     search: true               # 参与关键字搜索
//! ```

use serde::Deserialize;

/// 模块配置
#[derive(Debug, Clone, Deserialize)]
pub struct ModuleConfig {
    /// 数据库表名，如 sys_products
    pub table_name: String,
    /// 资源名 (snake_case 单数)，如 product，用于 API 路径
    pub resource: String,
    /// 中文模块名，如 "产品管理"
    pub module_cn: String,
    /// Element Plus 图标名，如 goods
    #[serde(default = "default_icon")]
    pub icon: String,
    /// 字段列表
    pub fields: Vec<FieldConfig>,
}

/// 字段配置
#[derive(Debug, Clone, Deserialize)]
pub struct FieldConfig {
    /// 字段名 (snake_case)
    pub name: String,
    /// 字段类型: string, text, i8, i32, i64, u64, f32, f64, bool, decimal, date, datetime
    #[serde(rename = "type")]
    pub field_type: String,
    /// 是否可空 (默认 true)
    #[serde(default = "default_true")]
    pub nullable: bool,
    /// 注释
    #[serde(default)]
    pub comment: String,
    /// 是否参与关键字搜索 (默认 false)
    #[serde(default)]
    pub search: bool,
}

fn default_icon() -> String {
    "document".into()
}

fn default_true() -> bool {
    true
}
