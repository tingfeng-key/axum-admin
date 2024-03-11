use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    pub id: i32,
    pub parent_id: i32,
    pub r#type: MenuType,
    pub title: String,
    pub icon: String,
    pub router_name: String,
    pub router_component: String,
    pub router_path: String,
    pub redirect: String,
    pub link: String,
    pub iframe: String,
    pub btn_auth: String,
    pub api_url: String,
    pub api_method: String,
    pub is_hide: i32,
    pub is_keep_alive: i32,
    pub is_affix: i32,
    pub sort: i32,
    pub created_at: String,
    pub updated_time: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MenuType {
    Menu = 1,
    Redirect = 2,
    Link = 3,
    Iframe = 4,
    BtnAuth = 5,
    Api = 6,
}

impl From<i32> for MenuType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Menu,
            2 => Self::Redirect,
            3 => Self::Link,
            4 => Self::Iframe,
            5 => Self::BtnAuth,
            6 => Self::Api,
            _ => Self::Menu,
        }
    }
}

impl From<MenuType> for i32 {
    fn from(value: MenuType) -> Self {
        match value {
            MenuType::Menu => 1,
            MenuType::Redirect => 2,
            MenuType::Link => 3,
            MenuType::Iframe => 4,
            MenuType::BtnAuth => 5,
            MenuType::Api => 6,
        }
    }
}
