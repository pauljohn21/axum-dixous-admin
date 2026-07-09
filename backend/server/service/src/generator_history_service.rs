//! 代码生成器历史记录 Service
//!
//! 功能:
//! - 历史 CRUD
//! - 回滚 (标记 flag=1, 可选删除表)
//! - 从数据库获取元数据 (数据库名/表名/字段信息)
//! - 根据数据库表结构生成 YAML 配置

use anyhow::{anyhow, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};

use model::dao::sys_generator_history;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::dto::sys_generator_history_dto::{
    ColumnInfo, DatabaseInfo, GenerateFromTableDTO, GeneratorRollbackDTO,
    SysGeneratorHistoryInsertDTO, SysGeneratorHistoryUpdateDTO, TableInfo,
};
use model::prelude::SysGeneratorHistory;
use utils::db_conn;

pub struct GeneratorHistoryService;

impl GeneratorHistoryService {
    /// 创建历史记录
    pub async fn insert(data: SysGeneratorHistoryInsertDTO) -> Result<sys_generator_history::Model> {
        let db = db_conn!();
        let active = sys_generator_history::ActiveModel {
            table_name: Set(data.table_name),
            resource: Set(data.resource),
            module_cn: Set(data.module_cn),
            request: Set(data.request),
            flag: Set(0),
            generated_files: Set(data.generated_files),
            ..Default::default()
        };
        let result = SysGeneratorHistory::insert(active).exec(db).await?;
        Self::get_by_id(result.last_insert_id).await
    }

    /// 分页查询历史记录
    pub async fn list(query: PageRequest) -> Result<PageResponse<sys_generator_history::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = SysGeneratorHistory::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(sys_generator_history::Column::TableName.contains(keyword))
                    .add(sys_generator_history::Column::Resource.contains(keyword))
                    .add(sys_generator_history::Column::ModuleCn.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(db)
            .await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    /// 按 ID 获取
    pub async fn get_by_id(id: u64) -> Result<sys_generator_history::Model> {
        SysGeneratorHistory::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("历史记录不存在"))
    }

    /// 获取历史记录的 YAML 配置
    pub async fn get_meta(id: u64) -> Result<String> {
        let record = Self::get_by_id(id).await?;
        Ok(record.request)
    }

    /// 更新历史记录
    pub async fn update(id: u64, data: SysGeneratorHistoryUpdateDTO) -> Result<sys_generator_history::Model> {
        let db = db_conn!();
        let record: sys_generator_history::ActiveModel = SysGeneratorHistory::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("历史记录不存在"))?
            .into();
        let mut updated = record;
        if let Some(v) = data.table_name { updated.table_name = Set(v); }
        if let Some(v) = data.resource { updated.resource = Set(v); }
        if let Some(v) = data.module_cn { updated.module_cn = Set(v); }
        if let Some(v) = data.request { updated.request = Set(v); }
        if let Some(v) = data.flag { updated.flag = Set(v); }
        if let Some(v) = data.generated_files { updated.generated_files = Set(Some(v)); }
        Ok(updated.update(db).await?)
    }

    /// 删除历史记录
    pub async fn delete(id: u64) -> Result<()> {
        SysGeneratorHistory::delete_by_id(id).exec(db_conn!()).await?;
        Ok(())
    }

    /// 回滚: 标记 flag=1, 可选删除数据库表
    pub async fn rollback(data: GeneratorRollbackDTO) -> Result<()> {
        let record = Self::get_by_id(data.id).await?;

        // 可选: 删除数据库表
        if data.delete_table {
            let db = db_conn!();
            let sql = format!("DROP TABLE IF EXISTS `{}`", record.table_name);
            db.execute(sea_orm::Statement::from_sql_and_values(
                db.get_database_backend(),
                &sql,
                [],
            ))
            .await?;
        }

        // 标记为已回滚
        Self::update(
            data.id,
            SysGeneratorHistoryUpdateDTO {
                flag: Some(1),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// 检测重复 (同表名且 flag=0)
    pub async fn check_repeat(table_name: &str) -> bool {
        let db = db_conn!();
        let count = SysGeneratorHistory::find()
            .filter(sys_generator_history::Column::TableName.eq(table_name))
            .filter(sys_generator_history::Column::Flag.eq(0))
            .count(db)
            .await
            .unwrap_or(0);
        count > 0
    }

    // ===== 从数据库创建 =====

    /// 获取所有数据库名
    pub async fn get_databases() -> Result<Vec<DatabaseInfo>> {
        let db = db_conn!();
        let rows = db
            .query_all(sea_orm::Statement::from_sql_and_values(
                db.get_database_backend(),
                "SELECT SCHEMA_NAME AS `database` FROM INFORMATION_SCHEMA.SCHEMATA",
                [],
            ))
            .await?;

        let mut databases = Vec::new();
        for row in rows {
            let database: String = row.try_get("", "database")?;
            databases.push(DatabaseInfo { database });
        }
        Ok(databases)
    }

    /// 获取指定数据库的所有表名
    pub async fn get_tables(db_name: &str) -> Result<Vec<TableInfo>> {
        let db = db_conn!();
        let rows = db
            .query_all(sea_orm::Statement::from_sql_and_values(
                db.get_database_backend(),
                "SELECT table_name FROM information_schema.tables WHERE table_schema = ?",
                [db_name.into()],
            ))
            .await?;

        let mut tables = Vec::new();
        for row in rows {
            let table_name: String = row.try_get("", "table_name")?;
            tables.push(TableInfo { table_name });
        }
        Ok(tables)
    }

    /// 获取指定表的字段信息
    pub async fn get_columns(db_name: &str, table_name: &str) -> Result<Vec<ColumnInfo>> {
        let db = db_conn!();
        let sql = r#"
            SELECT 
                c.COLUMN_NAME AS column_name,
                c.DATA_TYPE AS data_type,
                CASE c.DATA_TYPE
                    WHEN 'longtext' THEN CAST(c.CHARACTER_MAXIMUM_LENGTH AS CHAR)
                    WHEN 'varchar' THEN CAST(c.CHARACTER_MAXIMUM_LENGTH AS CHAR)
                    WHEN 'double' THEN CONCAT_WS(',', c.NUMERIC_PRECISION, c.NUMERIC_SCALE)
                    WHEN 'decimal' THEN CONCAT_WS(',', c.NUMERIC_PRECISION, c.NUMERIC_SCALE)
                    WHEN 'int' THEN CAST(c.NUMERIC_PRECISION AS CHAR)
                    WHEN 'bigint' THEN CAST(c.NUMERIC_PRECISION AS CHAR)
                    ELSE ''
                END AS data_type_long,
                c.COLUMN_COMMENT AS column_comment,
                CASE WHEN kcu.COLUMN_NAME IS NOT NULL THEN 1 ELSE 0 END AS primary_key,
                c.ORDINAL_POSITION AS ordinal_position
            FROM 
                INFORMATION_SCHEMA.COLUMNS c
            LEFT JOIN 
                INFORMATION_SCHEMA.KEY_COLUMN_USAGE kcu 
            ON 
                c.TABLE_SCHEMA = kcu.TABLE_SCHEMA 
                AND c.TABLE_NAME = kcu.TABLE_NAME 
                AND c.COLUMN_NAME = kcu.COLUMN_NAME 
                AND kcu.CONSTRAINT_NAME = 'PRIMARY'
            WHERE 
                c.TABLE_NAME = ? 
                AND c.TABLE_SCHEMA = ?
            ORDER BY 
                c.ORDINAL_POSITION
        "#;

        let rows = db
            .query_all(sea_orm::Statement::from_sql_and_values(
                db.get_database_backend(),
                sql,
                [table_name.into(), db_name.into()],
            ))
            .await?;

        let mut columns = Vec::new();
        for row in rows {
            let column_name: String = row.try_get("", "column_name")?;
            let data_type: String = row.try_get("", "data_type")?;
            let data_type_long: String = row.try_get("", "data_type_long").unwrap_or_default();
            let column_comment: String = row.try_get("", "column_comment").unwrap_or_default();
            let primary_key: i32 = row.try_get("", "primary_key").unwrap_or(0);
            let ordinal_position: i32 = row.try_get("", "ordinal_position").unwrap_or(0);

            columns.push(ColumnInfo {
                column_name,
                data_type,
                data_type_long,
                column_comment,
                primary_key: primary_key == 1,
                ordinal_position,
            });
        }
        Ok(columns)
    }

    /// 根据 MySQL 字段类型映射到 generator 支持的类型
    fn map_mysql_type(mysql_type: &str) -> &'static str {
        match mysql_type.to_lowercase().as_str() {
            "varchar" | "char" | "tinytext" | "mediumtext" | "longtext" => "string",
            "text" => "text",
            "tinyint" => "i8",
            "smallint" => "i16",
            "int" | "integer" | "mediumint" => "i32",
            "bigint" => "i64",
            "float" => "f32",
            "double" => "f64",
            "decimal" | "numeric" => "decimal",
            "bit" | "bool" | "boolean" => "bool",
            "date" => "date",
            "datetime" | "timestamp" => "datetime",
            "json" => "json",
            _ => "string",
        }
    }

    /// 将 snake_case 转为 PascalCase
    fn to_pascal(s: &str) -> String {
        s.split('_')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect()
    }

    /// 根据数据库表结构生成 JSON 配置
    pub async fn generate_from_table(data: GenerateFromTableDTO) -> Result<String> {
        let columns = Self::get_columns(&data.db_name, &data.table_name).await?;

        let table_name = &data.table_name;
        // 去掉 sys_ 前缀得到资源名
        let resource = table_name
            .strip_prefix("sys_")
            .unwrap_or(table_name)
            .trim_end_matches('s')
            .to_string();
        let module_cn = format!("{}管理", Self::to_pascal(&resource));

        // 构建 JSON 配置 (与前端 GeneratorConfig 结构对应)
        let mut fields_json = Vec::new();
        for col in &columns {
            // 跳过系统字段
            if matches!(
                col.column_name.as_str(),
                "id" | "created_at" | "updated_at" | "deleted_at"
            ) {
                continue;
            }

            let field_type = Self::map_mysql_type(&col.data_type);
            let comment = if col.column_comment.is_empty() {
                col.column_name.clone()
            } else {
                col.column_comment.clone()
            };

            fields_json.push(serde_json::json!({
                "name": col.column_name,
                "type": field_type,
                "nullable": true,
                "comment": comment,
                "search": false,
                "search_type": "",
                "require": false,
                "default_value": "",
                "form": true,
                "table": true,
                "desc": true,
                "sort": false,
                "primary_key": col.primary_key,
                "enum_values": ""
            }));
        }

        let config = serde_json::json!({
            "table_name": table_name,
            "resource": resource,
            "module_cn": module_cn,
            "icon": "document",
            "description": "",
            "generate_backend": true,
            "generate_frontend": true,
            "batch_delete": true,
            "fields": fields_json
        });

        Ok(serde_json::to_string_pretty(&config)?)
    }
}
