use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysOperationRecordInsertDTO {
    pub ip: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<i64>,
    pub latency: Option<i64>,
    pub agent: Option<String>,
    pub error_message: Option<String>,
    pub body: Option<String>,
    pub resp: Option<String>,
    pub user_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysOperationRecordUpdateDTO {
    pub ip: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<i64>,
    pub latency: Option<i64>,
    pub agent: Option<String>,
    pub error_message: Option<String>,
    pub body: Option<String>,
    pub resp: Option<String>,
    pub user_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysOperationRecordQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}
