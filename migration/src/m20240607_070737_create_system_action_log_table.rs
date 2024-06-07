use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemActionLog::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemActionLog::Id))
                    .col(integer(SystemActionLog::UserId))
                    .col(integer(SystemActionLog::MenuId))
                    .col(string(SystemActionLog::MenuNames).default(""))
                    .col(string(SystemActionLog::IpAddress))
                    .col(string(SystemActionLog::IpAddressName).default(""))
                    .col(string(SystemActionLog::BrowserAgent).default(""))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemActionLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemActionLog {
    Table,
    Id,
    UserId,
    MenuId,
    MenuNames,
    IpAddress,
    IpAddressName,
    BrowserAgent,
}
