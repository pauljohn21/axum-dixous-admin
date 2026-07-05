use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysDataRoleInsertDTO {
    pub sys_role_role_id: u64,
    pub data_role_id_role_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysDataRoleQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}
