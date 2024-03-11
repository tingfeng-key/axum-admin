use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub owner_uid: i32,
    pub parent_uid: i32,
    pub uid: i32,
    pub level: i32,
    pub created_at: String,
}
