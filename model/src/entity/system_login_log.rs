use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Entity {
    id: i32,
    user_id: i32,
    ip_address: String,
    ip_address_name: String,
    browser_agent: String,
    created_at: String,
}
