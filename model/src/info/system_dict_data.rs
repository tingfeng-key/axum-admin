use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub dict_id: i32,
    pub label: String,
    pub value: i32,
    pub status: i32,
    pub sort: i32,
    pub remark: String,
    pub created_at: String,
}
