use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Info {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub person_name: String,
    pub person_phone: String,
    pub person_email: String,
    pub describe: String,
    pub status: i32,
    pub sort: i32,
    pub created_at: String,
}
