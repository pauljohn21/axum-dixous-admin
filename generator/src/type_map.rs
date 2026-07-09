//! 字段类型映射
//!
//! 将配置文件中的字段类型映射到后端 Rust 类型、SeaORM 列类型、前端 Rust 类型

use crate::config::FieldConfig;

/// 类型映射信息
#[allow(dead_code)]
pub struct TypeMapping {
    /// 后端 Rust 类型 (如 String, i32, bool)
    pub backend_rust: &'static str,
    /// SeaORM 列类型方法名 (如 string, text, integer, tiny_integer)
    pub sea_orm_col: &'static str,
    /// 前端 Rust 类型 (如 String, i32, bool)
    pub frontend_rust: &'static str,
    /// 是否为字符串类型 (用于表单输入)
    pub is_string: bool,
    /// 是否为数值类型 (用于表单转换)
    pub is_numeric: bool,
    /// 是否为布尔类型
    pub is_bool: bool,
    /// 是否为时间类型
    pub is_time: bool,
    /// 是否为 JSON/数组类型 (需要特殊处理)
    pub is_json: bool,
    /// 是否为枚举类型
    pub is_enum: bool,
}

/// 获取字段类型映射
pub fn get_type_mapping(field_type: &str) -> TypeMapping {
    match field_type {
        "string" => TypeMapping {
            backend_rust: "String",
            sea_orm_col: "string",
            frontend_rust: "String",
            is_string: true,
            is_numeric: false,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "text" => TypeMapping {
            backend_rust: "String",
            sea_orm_col: "text",
            frontend_rust: "String",
            is_string: true,
            is_numeric: false,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "i8" => TypeMapping {
            backend_rust: "i8",
            sea_orm_col: "tiny_integer",
            frontend_rust: "i32",
            is_string: false,
            is_numeric: true,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "i16" => TypeMapping {
            backend_rust: "i16",
            sea_orm_col: "small_integer",
            frontend_rust: "i32",
            is_string: false,
            is_numeric: true,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "i32" => TypeMapping {
            backend_rust: "i32",
            sea_orm_col: "integer",
            frontend_rust: "i32",
            is_string: false,
            is_numeric: true,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "i64" => TypeMapping {
            backend_rust: "i64",
            sea_orm_col: "big_integer",
            frontend_rust: "i32",
            is_string: false,
            is_numeric: true,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "u64" => TypeMapping {
            backend_rust: "u64",
            sea_orm_col: "big_unsigned",
            frontend_rust: "i32",
            is_string: false,
            is_numeric: true,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "f32" => TypeMapping {
            backend_rust: "f32",
            sea_orm_col: "float",
            frontend_rust: "f32",
            is_string: false,
            is_numeric: true,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "f64" => TypeMapping {
            backend_rust: "f64",
            sea_orm_col: "double",
            frontend_rust: "f64",
            is_string: false,
            is_numeric: true,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "bool" => TypeMapping {
            backend_rust: "bool",
            sea_orm_col: "boolean",
            frontend_rust: "bool",
            is_string: false,
            is_numeric: false,
            is_bool: true,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "decimal" => TypeMapping {
            backend_rust: "String",
            sea_orm_col: "decimal_len",
            frontend_rust: "String",
            is_string: true,
            is_numeric: false,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
        "date" => TypeMapping {
            backend_rust: "String",
            sea_orm_col: "date",
            frontend_rust: "String",
            is_string: true,
            is_numeric: false,
            is_bool: false,
            is_time: true,
            is_json: false,
            is_enum: false,
        },
        "datetime" => TypeMapping {
            backend_rust: "String",
            sea_orm_col: "date_time",
            frontend_rust: "String",
            is_string: true,
            is_numeric: false,
            is_bool: false,
            is_time: true,
            is_json: false,
            is_enum: false,
        },
        "json" => TypeMapping {
            backend_rust: "serde_json::Value",
            sea_orm_col: "json",
            frontend_rust: "serde_json::Value",
            is_string: false,
            is_numeric: false,
            is_bool: false,
            is_time: false,
            is_json: true,
            is_enum: false,
        },
        "array" => TypeMapping {
            backend_rust: "Vec<String>",
            sea_orm_col: "json",
            frontend_rust: "Vec<String>",
            is_string: false,
            is_numeric: false,
            is_bool: false,
            is_time: false,
            is_json: true,
            is_enum: false,
        },
        "enum" => TypeMapping {
            backend_rust: "String",
            sea_orm_col: "string",
            frontend_rust: "String",
            is_string: true,
            is_numeric: false,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: true,
        },
        _ => TypeMapping {
            backend_rust: "String",
            sea_orm_col: "string",
            frontend_rust: "String",
            is_string: true,
            is_numeric: false,
            is_bool: false,
            is_time: false,
            is_json: false,
            is_enum: false,
        },
    }
}

/// 生成后端 DAO 字段类型 (总是 Option<T>，除了 id 和非空字段)
pub fn backend_field_type(field: &FieldConfig) -> String {
    let mapping = get_type_mapping(&field.field_type);
    if field.nullable {
        format!("Option<{}>", mapping.backend_rust)
    } else {
        mapping.backend_rust.to_string()
    }
}

/// 生成前端 Model 字段类型 (总是 Option<T>，除了 id)
pub fn frontend_field_type(field: &FieldConfig) -> String {
    let mapping = get_type_mapping(&field.field_type);
    format!("Option<{}>", mapping.frontend_rust)
}

/// 生成前端 InsertDTO 字段类型
/// 字符串类型: String (必填)，其他类型: Option<T>
pub fn frontend_insert_type(field: &FieldConfig) -> String {
    let mapping = get_type_mapping(&field.field_type);
    if mapping.is_string {
        "String".to_string()
    } else {
        format!("Option<{}>", mapping.frontend_rust)
    }
}

/// 生成前端 UpdateDTO 字段类型 (总是 Option<T>)
pub fn frontend_update_type(field: &FieldConfig) -> String {
    let mapping = get_type_mapping(&field.field_type);
    format!("Option<{}>", mapping.frontend_rust)
}

/// 搜索类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum SearchType {
    Like,
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
    Between,
    None,
}

impl SearchType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "like" => SearchType::Like,
            "eq" | "=" => SearchType::Eq,
            "ne" | "!=" => SearchType::Ne,
            "gt" | ">" => SearchType::Gt,
            "lt" | "<" => SearchType::Lt,
            "gte" | ">=" => SearchType::Gte,
            "lte" | "<=" => SearchType::Lte,
            "between" => SearchType::Between,
            _ => SearchType::None,
        }
    }

    /// 是否需要范围查询 (两个值)
    pub fn is_range(&self) -> bool {
        matches!(self, SearchType::Between)
    }
}
