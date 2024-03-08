pub mod cache;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    SerializeJson(serde_json::Error),
    CacheNotFound,
    #[cfg(feature = "cache-database")]
    Model(model::Error),
}

impl From<serde_json::Error> for ServiceError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializeJson(value)
    }
}
#[cfg(feature = "cache-database")]
impl From<model::Error> for ServiceError {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}
