//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "system_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub nickname: String,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub phone: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    pub sex: i32,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text")]
    pub salt: String,
    #[sea_orm(column_type = "Text")]
    pub describe: String,
    #[sea_orm(column_type = "custom(\"DATETIME\")", nullable)]
    pub expire_time: Option<String>,
    pub status: i32,
    #[sea_orm(column_type = "Text")]
    pub last_login_ip: String,
    #[sea_orm(column_type = "custom(\"DATETIME\")", nullable)]
    pub last_login_time: Option<String>,
    #[sea_orm(column_type = "custom(\"DATETIME\")")]
    pub created_at: String,
    #[sea_orm(column_type = "custom(\"DATETIME\")")]
    pub updated_at: String,
    #[sea_orm(column_type = "custom(\"DATETIME\")", nullable)]
    pub deleted_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::system_action_logs::Entity")]
    SystemActionLogs,
    #[sea_orm(
        belongs_to = "super::system_depts::Entity",
        from = "Column::DeptId",
        to = "super::system_depts::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    SystemDepts,
    #[sea_orm(has_many = "super::system_login_logs::Entity")]
    SystemLoginLogs,
    #[sea_orm(
        belongs_to = "super::system_roles::Entity",
        from = "Column::RoleId",
        to = "super::system_roles::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    SystemRoles,
}

impl Related<super::system_action_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SystemActionLogs.def()
    }
}

impl Related<super::system_depts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SystemDepts.def()
    }
}

impl Related<super::system_login_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SystemLoginLogs.def()
    }
}

impl Related<super::system_roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SystemRoles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}