use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Member::Table)
                    .if_not_exists()
                    .col(pk_auto(Member::Id))
                    .col(string(Member::Nickname).default(""))
                    .col(string(Member::Avatar).default(""))
                    .col(string(Member::UserName).default(""))
                    .col(string(Member::Email).default(""))
                    .col(string(Member::Mobile).default(""))
                    .col(string(Member::Password).default(""))
                    .col(string(Member::Salt).default(""))
                    .col(float(Member::Balance).default(0.0))
                    .col(float(Member::Integral).default(0.0))
                    .col(string(Member::Remark).default(""))
                    .col(string(Member::Status).default(""))
                    .col(string(Member::LastLoginIp).default(""))
                    .col(string(Member::LastLoginTime).default(""))
                    .col(string(Post::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Member::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Member {
    Table,
    Id,
    Nickname,
    Avatar,
    UserName,
    Email,
    Mobile,
    Password,
    Salt,
    Balance,
    Integral,
    Remark,
    Status,
    LastLoginIp,
    LastLoginTime,
    InviteCode,
}
