use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Info {
    pub id: i32,
    pub name: String,
    pub sign: String,
    pub describe: String,
    pub status: i32,
    pub sort: i32,
    pub created_at: String,
    pub menu_ids: Vec<i32>,
}
