use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(pk_auto(Category::Id))
                    .col(integer(Category::ParentId).default(0))
                    .col(string(Category::Type).default(1))
                    .col(string(Category::Name))
                    .col(string(Category::Icon).default(""))
                    .col(integer(Category::Status).default(1))
                    .col(integer(Category::Sort).default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    ParentId,
    Type,
    Name,
    Icon,
    Status,
    Sort,
}
