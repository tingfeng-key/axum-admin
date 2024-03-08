pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerializeJson(serde_json::Error),
    CacheNotFound,
    #[cfg(feature = "cache-database")]
    Model(model::Error),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializeJson(value)
    }
}
#[cfg(feature = "cache-database")]
impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Self::Model(value)
    }
}
