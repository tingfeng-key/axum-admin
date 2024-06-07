use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemCache::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemCache::Id))
                    .col(integer(SystemCache::Type))
                    .col(string(SystemCache::Key))
                    .col(string(SystemCache::Value))
                    .col(string(SystemCache::Attach))
                    .col(integer_null(SystemCache::ValidTimeLength))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemCache::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemCache {
    Table,
    Id,
    Type,
    Key,
    Value,
    Attach,
    ValidTimeLength,
}
