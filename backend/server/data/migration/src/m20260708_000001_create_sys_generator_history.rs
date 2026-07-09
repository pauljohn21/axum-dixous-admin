use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysGeneratorHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SysGeneratorHistory::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SysGeneratorHistory::CreatedAt).date_time().null())
                    .col(ColumnDef::new(SysGeneratorHistory::UpdatedAt).date_time().null())
                    .col(ColumnDef::new(SysGeneratorHistory::DeletedAt).date_time().null())
                    .col(
                        ColumnDef::new(SysGeneratorHistory::TableName)
                            .string_len(255)
                            .not_null()
                            .comment("表名"),
                    )
                    .col(
                        ColumnDef::new(SysGeneratorHistory::Resource)
                            .string_len(255)
                            .not_null()
                            .comment("资源名"),
                    )
                    .col(
                        ColumnDef::new(SysGeneratorHistory::ModuleCn)
                            .string_len(255)
                            .not_null()
                            .comment("中文模块名"),
                    )
                    .col(
                        ColumnDef::new(SysGeneratorHistory::Request)
                            .text()
                            .not_null()
                            .comment("前端传入的完整 YAML 配置"),
                    )
                    .col(
                        ColumnDef::new(SysGeneratorHistory::Flag)
                            .integer()
                            .not_null()
                            .default(0)
                            .comment("标记: 0=创建, 1=回滚"),
                    )
                    .col(
                        ColumnDef::new(SysGeneratorHistory::GeneratedFiles)
                            .text()
                            .null()
                            .comment("生成的文件列表 JSON"),
                    )
                    .index(
                        Index::create()
                            .name("idx_generator_history_deleted_at")
                            .col(SysGeneratorHistory::DeletedAt),
                    )
                    .index(
                        Index::create()
                            .name("idx_generator_history_table_name")
                            .col(SysGeneratorHistory::TableName),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysGeneratorHistory::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum SysGeneratorHistory {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    TableName,
    Resource,
    ModuleCn,
    Request,
    Flag,
    GeneratedFiles,
}
