use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Default, ToSchema)]
pub struct SysUserRoleAddDto {
    pub user_id: i32,
    pub role_id: i32,
}