use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemRole::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemRole::Id))
                    .col(string(SystemRole::Name))
                    .col(string_uniq(SystemRole::Sign))
                    .col(string(SystemRole::Describe).default(""))
                    .col(integer(SystemRole::Status).default(1))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemRole {
    Table,
    Id,
    Name,
    Sign,
    Sort,
    Describe,
    Status,
}
