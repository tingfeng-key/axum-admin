pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240607_060550_system_user;
mod m20240607_060628_create_system_user_table;
mod m20240607_063342_create_system_user_table;
mod m20240607_065622_create_system_role_table;
mod m20240607_065629_create_system_dept_table;
mod m20240607_065938_system_user;
mod m20240607_070649_create_system_menu_table;
mod m20240607_070705_create_system_role_menu_table;
mod m20240607_070719_create_system_dict_table;
mod m20240607_070723_create_system_dict_data_table;
mod m20240607_070730_create_system_login_log_table;
mod m20240607_070737_create_system_action_log_table;
mod m20240607_070749_create_system_cache_table;
mod m20240607_070800_create_system_member_table;
mod m20240607_070811_create_system_category_table;
mod m20240607_070827_create_system_article_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240607_060550_system_user::Migration),
            Box::new(m20240607_060628_create_system_user_table::Migration),
            Box::new(m20240607_063342_create_system_user_table::Migration),
            Box::new(m20240607_065622_create_system_role_table::Migration),
            Box::new(m20240607_065629_create_system_dept_table::Migration),
            Box::new(m20240607_065938_system_user::Migration),
            Box::new(m20240607_070649_create_system_menu_table::Migration),
            Box::new(m20240607_070705_create_system_role_menu_table::Migration),
            Box::new(m20240607_070719_create_system_dict_table::Migration),
            Box::new(m20240607_070723_create_system_dict_data_table::Migration),
            Box::new(m20240607_070730_create_system_login_log_table::Migration),
            Box::new(m20240607_070737_create_system_action_log_table::Migration),
            Box::new(m20240607_070749_create_system_cache_table::Migration),
            Box::new(m20240607_070800_create_system_member_table::Migration),
            Box::new(m20240607_070811_create_system_category_table::Migration),
            Box::new(m20240607_070827_create_system_article_table::Migration),
        ]
    }
}
