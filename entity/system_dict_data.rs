//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "system_dict_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub dict_id: i32,
    #[sea_orm(column_type = "Text")]
    pub label: String,
    pub value: i32,
    #[sea_orm(column_type = "Text")]
    pub remark: String,
    pub status: i32,
    pub sort: i32,
    #[sea_orm(column_type = "custom(\"DATETIME\")")]
    pub created_at: String,
    #[sea_orm(column_type = "custom(\"DATETIME\")")]
    pub updated_at: String,
    #[sea_orm(column_type = "custom(\"DATETIME\")", nullable)]
    pub deleted_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::system_dicts::Entity",
        from = "Column::DictId",
        to = "super::system_dicts::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    SystemDicts,
}

impl Related<super::system_dicts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SystemDicts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}