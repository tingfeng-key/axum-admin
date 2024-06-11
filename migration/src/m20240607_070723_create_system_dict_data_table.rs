use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemDictData::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemDictData::Id))
                    .col(integer(SystemDictData::DictId))
                    .col(string(SystemDictData::Label))
                    .col(integer(SystemDictData::Value))
                    .col(string(SystemDictData::Remark))
                    .col(integer(SystemDictData::Status))
                    .col(integer(SystemDictData::Sort))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemDictData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemDictData {
    Table,
    Id,
    DictId,
    Label,
    Value,
    Remark,
    Status,
    Sort,
}
