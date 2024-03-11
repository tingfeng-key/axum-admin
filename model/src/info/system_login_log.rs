use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub user_id: i32,
    pub ip_address: String,
    pub ip_address_name: String,
    pub browser_agent: String,
    pub created_at: String,
}
