use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JwtBlacklists::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JwtBlacklists::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(JwtBlacklists::CreatedAt).date_time().null())
                    .col(ColumnDef::new(JwtBlacklists::UpdatedAt).date_time().null())
                    .col(ColumnDef::new(JwtBlacklists::DeletedAt).date_time().null())
                    .col(
                        ColumnDef::new(JwtBlacklists::Jwt)
                            .text()
                            .null()
                            .comment("jwt"),
                    )
                    .index(
                        Index::create()
                            .name("idx_jwt_blacklists_deleted_at")
                            .col(JwtBlacklists::DeletedAt),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(JwtBlacklists::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum JwtBlacklists {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Jwt,
}