use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub user_id: i32,
    pub r#type: i32,
    pub pm: i32,
    pub number: f64,
    pub created_at: String,
}
