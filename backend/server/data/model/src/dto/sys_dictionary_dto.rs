use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysDictionaryInsertDTO {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub status: Option<i8>,
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysDictionaryUpdateDTO {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub status: Option<i8>,
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysDictionaryQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}
