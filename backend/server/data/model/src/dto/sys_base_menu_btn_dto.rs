use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysBaseMenuBtnInsertDTO {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub sys_base_menu_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysBaseMenuBtnUpdateDTO {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub sys_base_menu_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysBaseMenuBtnQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}
