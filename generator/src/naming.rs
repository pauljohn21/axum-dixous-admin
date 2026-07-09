//! 命名转换工具
//!
//! 在 snake_case、PascalCase 之间转换

use crate::config::ModuleConfig;

/// snake_case → PascalCase: `sys_products` → `SysProducts`
pub fn to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// 模块命名上下文 — 从配置中派生所有命名变体
///
/// 命名规范 (参照现有 sys_dictionaries 模块):
/// - 表名 (复数):     sys_products
/// - DAO 文件/模块:    sys_products          (复数，与表名一致)
/// - Entity 别名:     SysProducts            (复数 PascalCase)
/// - DTO 文件/模块:    sys_product_dto       (单数)
/// - DTO 结构体:      SysProductInsertDTO   (单数 + Sys 前缀)
/// - Service 文件:    sys_product_service   (单数)
/// - Service 结构体:  SysProductService     (单数 + Sys 前缀)
/// - API 文件:        product_api           (resource 名)
/// - API 路径:        /api/product          (resource 名)
/// - 前端 Model:      SysProduct            (单数 + Sys 前缀)
/// - 前端组件:        ProductManage          (PascalCase(resource) + Manage)
pub struct Naming {
    #[allow(dead_code)]
    /// sys_products (复数表名)
    pub table_name: String,
    /// product (单数资源名)
    pub resource: String,
    /// sys_products (DAO 文件名，复数，与表名一致)
    pub dao_file: String,
    /// sys_product_dto (DTO 文件名，单数)
    pub dto_file: String,
    /// sys_product_service (Service 文件名，单数)
    pub service_file: String,
    /// product_api (API 文件名)
    pub api_file: String,
    /// product (前端 model/api 文件名)
    pub fe_file: String,
    /// product_manage (前端组件文件名)
    pub component_file: String,
    /// SysProducts (Entity 别名，复数 PascalCase)
    pub entity_name: String,
    /// SysProduct (单数 PascalCase + Sys 前缀，用于 DTO/Service 结构体名)
    pub pascal_singular: String,
    /// Product (PascalCase(resource)，无 Sys 前缀，用于 TKey 和组件名)
    pub pascal_resource: String,
    /// ProductManage (组件名)
    pub component_name: String,
    /// 产品管理 (中文模块名)
    pub module_cn: String,
    /// Element Plus 图标
    #[allow(dead_code)]
    pub icon: String,
}

impl Naming {
    pub fn from_config(config: &ModuleConfig) -> Self {
        let resource = config.resource.clone();

        // Entity 名从 table_name 派生: sys_products → SysProducts (复数)
        let entity_name = to_pascal(&config.table_name);

        // 单数 PascalCase + Sys 前缀: Sys + PascalCase(resource) → SysProduct
        let pascal_singular = format!("Sys{}", to_pascal(&resource));
        // PascalCase(resource) 无 Sys 前缀: Product
        let pascal_resource = to_pascal(&resource);

        // 组件名: ProductManage (无 Sys 前缀)
        let component_name = format!("{}Manage", to_pascal(&resource));

        Self {
            table_name: config.table_name.clone(),
            resource: resource.clone(),
            dao_file: config.table_name.clone(),          // sys_products (复数)
            dto_file: format!("sys_{}_dto", resource),     // sys_product_dto (单数)
            service_file: format!("sys_{}_service", resource), // sys_product_service (单数)
            api_file: format!("{}_api", resource),         // product_api
            fe_file: resource.clone(),                     // product
            component_file: format!("{}_manage", resource), // product_manage
            entity_name,                                    // SysProducts
            pascal_singular,                                // SysProduct
            pascal_resource,                                // Product
            component_name,                                 // ProductManage
            module_cn: config.module_cn.clone(),
            icon: config.icon.clone(),
        }
    }
}
