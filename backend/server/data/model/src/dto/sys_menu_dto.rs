use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysMenuInsertDTO {
    pub menu_level: Option<u64>,
    pub parent_id: Option<u64>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub hidden: Option<u8>,
    pub component: Option<String>,
    pub sort: Option<i64>,
    pub active_name: Option<String>,
    pub keep_alive: Option<i8>,
    pub default_menu: Option<i8>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub close_tab: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysMenuUpdateDTO {
    pub menu_level: Option<u64>,
    pub parent_id: Option<u64>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub hidden: Option<u8>,
    pub component: Option<String>,
    pub sort: Option<i64>,
    pub active_name: Option<String>,
    pub keep_alive: Option<i8>,
    pub default_menu: Option<i8>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub close_tab: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysMenuQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub parent_id: Option<u64>,
}
