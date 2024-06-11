use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemDept::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemDept::Id))
                    .col(integer(SystemDept::ParentId).default(0))
                    .col(string(SystemDept::Name))
                    .col(string(SystemDept::PersonName).default(""))
                    .col(string(SystemDept::PersonPhone).default(""))
                    .col(string(SystemDept::PersonEmail).default(""))
                    .col(string(SystemDept::Describe).default(""))
                    .col(integer(SystemDept::Status).default(1))
                    .col(integer(SystemDept::Sort).default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemDept::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemDept {
    Table,
    Id,
    ParentId,
    Name,
    PersonName,
    PersonPhone,
    PersonEmail,
    Describe,
    Status,
    Sort,
}
