use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemUser::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemUser::Id))
                    .col(string_uniq(SystemUser::Username))
                    .col(string(SystemUser::Nickname).default(""))
                    .col(integer(SystemUser::RoleId).default(0))
                    .col(integer(SystemUser::DeptId).default(0))
                    .col(string(SystemUser::Phone).default(""))
                    .col(string(SystemUser::Email).default(""))
                    .col(integer(SystemUser::Sex).default(1))
                    .col(string(SystemUser::Password).default(""))
                    .col(string(SystemUser::Salt).default(""))
                    .col(string(SystemUser::Describe).default(""))
                    .col(date_time_null(SystemUser::ExpireTime))
                    .col(integer(SystemUser::Status).default(0))
                    .col(string(SystemUser::LastLoginIp).default(""))
                    .col(date_time_null(SystemUser::LastLoginTime))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemUser {
    Table,
    Id,
    Username,
    Nickname,
    RoleId,
    DeptId,
    Phone,
    Email,
    Sex,
    Password,
    Salt,
    Describe,
    ExpireTime,
    Status,
    LastLoginIp,
    LastLoginTime,
}
