//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "system_role_menus")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role_id: i32,
    pub menu_id: i32,
    #[sea_orm(column_type = "custom(\"DATETIME\")", nullable)]
    pub deleted_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::system_menus::Entity",
        from = "Column::MenuId",
        to = "super::system_menus::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    SystemMenus,
    #[sea_orm(
        belongs_to = "super::system_roles::Entity",
        from = "Column::RoleId",
        to = "super::system_roles::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    SystemRoles,
}

impl Related<super::system_menus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SystemMenus.def()
    }
}

impl Related<super::system_roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SystemRoles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}