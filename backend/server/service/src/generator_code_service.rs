//! 代码生成 Service
//!
//! 根据配置生成前后端代码，支持预览和实际写入文件。

use anyhow::Result;
use chrono::Local;
use serde::Serialize;
use utoipa::ToSchema;

use model::dto::sys_generator_history_dto::PreviewCodeDTO;

/// 生成的代码文件
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GeneratedFile {
    pub file_name: String,
    pub file_path: String,
    pub content: String,
    pub file_type: String, // "rust" | "typescript" | "sql"
}

/// 代码预览响应
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PreviewCodeResponse {
    pub backend_files: Vec<GeneratedFile>,
    pub frontend_files: Vec<GeneratedFile>,
}

pub struct GeneratorCodeService;

impl GeneratorCodeService {
    /// 预览代码 - 根据 JSON 配置生成所有代码文件内容
    pub async fn preview_code(data: PreviewCodeDTO) -> Result<PreviewCodeResponse> {
        let config: serde_json::Value = serde_json::from_str(&data.config_json)?;
        
        let mut backend_files = Vec::new();
        let mut frontend_files = Vec::new();

        // 解析配置
        let table_name = config["table_name"].as_str().unwrap_or("");
        let resource = config["resource"].as_str().unwrap_or("");
        let generate_backend = config["generate_backend"].as_bool().unwrap_or(true);
        let generate_frontend = config["generate_frontend"].as_bool().unwrap_or(true);

        if generate_backend {
            backend_files.push(GeneratedFile {
                file_name: format!("m{}_create_{}.rs", Self::timestamp(), table_name),
                file_path: format!("data/migration/src/m{}_create_{}.rs", Self::timestamp(), table_name),
                content: "// Migration code will be generated here\n// TODO: Implement full code generation".to_string(),
                file_type: "rust".to_string(),
            });
            backend_files.push(GeneratedFile {
                file_name: format!("{}.rs", resource),
                file_path: format!("data/model/src/dao/{}.rs", resource),
                content: "// DAO code will be generated here".to_string(),
                file_type: "rust".to_string(),
            });
        }

        if generate_frontend {
            frontend_files.push(GeneratedFile {
                file_name: format!("{}_manage.rs", resource),
                file_path: format!("src/components/{}_manage.rs", resource),
                content: "// Frontend component will be generated here".to_string(),
                file_type: "rust".to_string(),
            });
        }

        Ok(PreviewCodeResponse {
            backend_files,
            frontend_files,
        })
    }

    fn timestamp() -> String {
        Local::now().format("%Y%m%d_%H%M%S").to_string()
    }
}
