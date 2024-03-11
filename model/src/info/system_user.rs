use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub role_id: Option<i32>,
    pub dept_id: Option<i32>,
    pub phone: String,
    pub email: String,
    pub sex: i32,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub salt: String,
    pub describe: String,
    pub expire_time: Option<String>,
    pub status: i32,
    pub created_at: String,
    pub last_login_ip: String,
    pub last_login_time: Option<String>,
}
