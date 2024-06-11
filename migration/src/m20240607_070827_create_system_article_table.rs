use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .if_not_exists()
                    .col(pk_auto(Article::Id))
                    .col(integer(Article::CategoryId).default(0))
                    .col(string(Article::Title))
                    .col(string(Article::Thumb).default(""))
                    .col(string(Article::Author).default(""))
                    .col(string(Article::Descption).default(""))
                    .col(string(Article::Content).default(""))
                    .col(integer(Article::Visit))
                    .col(integer(Article::IsElite).default(0))
                    .col(integer(Article::IsTop).default(0))
                    .col(integer(Article::IsHot).default(0))
                    .col(integer(Article::Status).default(1))
                    .col(integer(Article::Sort).default(0))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("article_category_id_fkey")
                            .from_tbl(Article::Table)
                            .from_col(Article::CategoryId)
                            .to_tbl(super::Category::Table)
                            .to_col(super::Category::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id,
    CategoryId,
    Title,
    Thumb,
    Author,
    Descption,
    Content,
    Visit,
    IsElite,
    IsTop,
    IsHot,
    Status,
    Sort,
}
