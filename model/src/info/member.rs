use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Info {
    pub id: i32,
    pub unique_code: String,
    pub email: String,
    pub mobile: String,
    pub nickname: String,
    pub avatar: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub salt: String,
    pub sex: i32,
    pub balance: f32,
    pub integral: i32,
    pub remark: String,
    pub status: i32,
    pub is_promoter: i32,
    pub last_login_ip: String,
    pub last_login_time: Option<String>,
    pub created_at: String,
}
