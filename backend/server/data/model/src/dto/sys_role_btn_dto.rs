use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysRoleBtnInsertDTO {
    pub role_id: u64,
    pub sys_menu_id: u64,
    pub sys_base_menu_btn_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysRoleBtnQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}
