use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemMenu::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemMenu::Id))
                    .col(integer(SystemMenu::ParentId).default(0))
                    .col(integer(SystemMenu::Type).default(1))
                    .col(string(SystemMenu::Name))
                    .col(string(SystemMenu::Icon).default(""))
                    .col(string(SystemMenu::RouterName).default(""))
                    .col(string(SystemMenu::RouterComponent).default(""))
                    .col(string(SystemMenu::RouterPath).default(""))
                    .col(string(SystemMenu::Redirect).default(""))
                    .col(string(SystemMenu::Link).default(""))
                    .col(string(SystemMenu::IFrame).default(""))
                    .col(string(SystemMenu::BtnAuth).default(""))
                    .col(string(SystemMenu::ApiUrl).default(""))
                    .col(string(SystemMenu::ApiMethod).default(""))
                    .col(integer(SystemMenu::IsHide).default(0))
                    .col(integer(SystemMenu::IsKeepAlive).default(1))
                    .col(integer(SystemMenu::IsAffix).default(0))
                    .col(integer(SystemMenu::Sort).default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemMenu::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemMenu {
    Table,
    Id,
    ParentId,
    Type,
    Name,
    Icon,
    RouterName,
    RouterComponent,
    RouterPath,
    Redirect,
    Link,
    IFrame,
    BtnAuth,
    ApiUrl,
    ApiMethod,
    IsHide,
    IsKeepAlive,
    IsAffix,
    Sort,
}
