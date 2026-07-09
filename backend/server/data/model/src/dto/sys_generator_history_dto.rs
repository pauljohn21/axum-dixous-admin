use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 创建历史记录
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysGeneratorHistoryInsertDTO {
    /// 表名
    pub table_name: String,
    /// 资源名
    pub resource: String,
    /// 中文模块名
    pub module_cn: String,
    /// 完整 JSON 配置
    pub request: String,
    /// 生成的文件列表 JSON (可选)
    pub generated_files: Option<String>,
}

/// 更新历史记录
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysGeneratorHistoryUpdateDTO {
    pub table_name: Option<String>,
    pub resource: Option<String>,
    pub module_cn: Option<String>,
    pub request: Option<String>,
    pub flag: Option<i32>,
    pub generated_files: Option<String>,
}

/// 查询历史记录
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysGeneratorHistoryQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}

/// 回滚请求
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct GeneratorRollbackDTO {
    /// 历史记录 ID
    pub id: u64,
    /// 是否删除数据库表
    pub delete_table: bool,
}

/// 从数据库创建 — 获取所有数据库名
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DatabaseInfo {
    pub database: String,
}

/// 从数据库创建 — 获取所有表名
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TableInfo {
    pub table_name: String,
}

/// 从数据库创建 — 获取表字段信息
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ColumnInfo {
    pub column_name: String,
    pub data_type: String,
    pub data_type_long: String,
    pub column_comment: String,
    pub primary_key: bool,
    pub ordinal_position: i32,
}

/// 从数据库创建 — 根据表结构生成 JSON 配置的请求
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct GenerateFromTableDTO {
    /// 数据库名
    pub db_name: String,
    /// 表名
    pub table_name: String,
}

/// 代码预览请求
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct PreviewCodeDTO {
    /// JSON 格式的配置
    pub config_json: String,
}

/// 生成的代码文件
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeneratedFile {
    pub file_name: String,
    pub file_path: String,
    pub content: String,
    pub file_type: String,
}

/// 代码预览响应
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PreviewCodeResponse {
    pub backend_files: Vec<GeneratedFile>,
    pub frontend_files: Vec<GeneratedFile>,
}
