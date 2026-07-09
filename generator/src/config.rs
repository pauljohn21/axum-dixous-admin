//! 配置文件解析模块
//!
//! YAML 配置文件格式：
//! ```yaml
//! table_name: sys_products       # 数据库表名 (sys_ 前缀)
//! resource: product               # 资源名 (snake_case 单数)
//! module_cn: "产品管理"            # 中文模块名
//! icon: goods                     # Element Plus 图标
//! description: "产品"              # Struct 中文描述 (可选, 默认同 module_cn)
//! generate_backend: true          # 是否生成后端 (可选, 默认 true)
//! generate_frontend: true         # 是否生成前端 (可选, 默认 true)
//! batch_delete: true              # 是否支持批量删除 (可选, 默认 true)
//! fields:
//!   - name: name
//!     type: string
//!     nullable: true
//!     comment: "产品名称"
//!     search: true               # 简化搜索 (关键字模糊匹配)
//!     # 或使用 search_type 精确控制搜索方式:
//!     # search_type: like        # like | eq | ne | gt | lt | gte | lte | between
//!     require: false             # 是否必填 (可选, 默认 false)
//!     default_value: ""          # 默认值 (可选)
//!     form: true                 # 是否在表单中显示 (可选, 默认 true)
//!     table: true                # 是否在表格中显示 (可选, 默认 true)
//!     desc: true                 # 是否在详情中显示 (可选, 默认 true)
//!     sort: false                # 是否可排序 (可选, 默认 false)
//!     primary_key: false         # 是否主键 (可选, 默认 false)
//! ```

use serde::Deserialize;

/// 模块配置
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
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
    /// Struct 中文描述 (可选, 默认同 module_cn)
    #[serde(default)]
    pub description: String,
    /// 是否生成后端代码 (可选, 默认 true)
    #[serde(default = "default_true")]
    pub generate_backend: bool,
    /// 是否生成前端代码 (可选, 默认 true)
    #[serde(default = "default_true")]
    pub generate_frontend: bool,
    /// 是否支持批量删除 (可选, 默认 true)
    #[serde(default = "default_true")]
    pub batch_delete: bool,
    /// 字段列表
    pub fields: Vec<FieldConfig>,
}

/// 字段配置
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct FieldConfig {
    /// 字段名 (snake_case)
    pub name: String,
    /// 字段类型: string, text, i8, i16, i32, i64, u64, f32, f64, bool, decimal, date, datetime, json, array, enum
    #[serde(rename = "type")]
    pub field_type: String,
    /// 是否可空 (默认 true)
    #[serde(default = "default_true")]
    pub nullable: bool,
    /// 数据库注释
    #[serde(default)]
    pub comment: String,
    /// 是否参与关键字搜索 (简化模式, 等同于 search_type: like) (默认 false)
    #[serde(default)]
    pub search: bool,
    /// 搜索类型: like, eq, ne, gt, lt, gte, lte, between (默认空 = 不搜索)
    /// 当 search 为 true 且 search_type 为空时, 自动使用 "like"
    #[serde(default)]
    pub search_type: String,
    /// 是否必填 (默认 false)
    #[serde(default)]
    pub require: bool,
    /// 默认值 (可选)
    #[serde(default)]
    pub default_value: String,
    /// 是否在表单中显示 (默认 true)
    #[serde(default = "default_true")]
    pub form: bool,
    /// 是否在表格中显示 (默认 true)
    #[serde(default = "default_true")]
    pub table: bool,
    /// 是否在详情中显示 (默认 true)
    #[serde(default = "default_true")]
    pub desc: bool,
    /// 是否可排序 (默认 false)
    #[serde(default)]
    pub sort: bool,
    /// 是否主键 (默认 false)
    #[serde(default)]
    pub primary_key: bool,
    /// 枚举值 (仅 type=enum 时使用, 逗号分隔, 如 "active,inactive")
    #[serde(default)]
    pub enum_values: String,
}

#[allow(dead_code)]
impl FieldConfig {
    /// 获取有效的搜索类型
    /// 如果 search=true 且 search_type 为空, 返回 "like"
    /// 如果 search_type 非空, 返回 search_type
    /// 否则返回空字符串
    pub fn effective_search_type(&self) -> &str {
        if !self.search_type.is_empty() {
            &self.search_type
        } else if self.search {
            "like"
        } else {
            ""
        }
    }

    /// 是否参与搜索
    pub fn is_searchable(&self) -> bool {
        !self.effective_search_type().is_empty()
    }

    /// 获取 JSON 字段名 (默认与 name 相同)
    pub fn json_name(&self) -> &str {
        &self.name
    }

    /// 获取中文描述 (优先使用 comment, 其次 name)
    pub fn desc_name(&self) -> &str {
        if self.comment.is_empty() {
            &self.name
        } else {
            &self.comment
        }
    }
}

fn default_icon() -> String {
    "document".into()
}

fn default_true() -> bool {
    true
}
