use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysRoleInsertDTO {
    pub en_name: String,
    pub cn_name: String,
    pub parent_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysRoleUpdateDTO {
    pub en_name: Option<String>,
    pub cn_name: Option<String>,
    pub parent_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysRoleQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}
