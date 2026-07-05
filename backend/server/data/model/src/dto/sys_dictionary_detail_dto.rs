use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysDictionaryDetailInsertDTO {
    pub label: Option<String>,
    pub value: Option<String>,
    pub extend: Option<String>,
    pub status: Option<i8>,
    pub sort: Option<i64>,
    pub sys_dictionary_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysDictionaryDetailUpdateDTO {
    pub label: Option<String>,
    pub value: Option<String>,
    pub extend: Option<String>,
    pub status: Option<i8>,
    pub sort: Option<i64>,
    pub sys_dictionary_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct SysDictionaryDetailQueryDTO {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
}
