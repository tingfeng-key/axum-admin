use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub name: String,
    pub sign: String,
    pub status: i32,
    pub remark: String,
    pub created_at: String,
}
