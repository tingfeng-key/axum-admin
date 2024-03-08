pub mod cache;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    SerializeJson(serde_json::Error),
    CacheNotFound,
    Model(model::Error),
}

impl From<serde_json::Error> for ServiceError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializeJson(value)
    }
}
impl From<model::Error> for ServiceError {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}
