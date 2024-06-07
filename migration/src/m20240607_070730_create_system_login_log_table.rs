use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemLoginLog::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemLoginLog::Id))
                    .col(integer(SystemLoginLog::Type).default(1))
                    .col(integer(SystemLoginLog::UserId))
                    .col(string(SystemLoginLog::IpAddress).default(""))
                    .col(string(SystemLoginLog::IpAddressName).default(""))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemLoginLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemLoginLog {
    Table,
    Id,
    Type,
    UserId,
    IpAddress,
    IpAddressName,
    BrowserAgent,
}
