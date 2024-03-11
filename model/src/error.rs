#[cfg(feature = "diesel_async")]
use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DataNotFound,
    #[cfg(feature = "diesel_async")]
    Pool(PoolError),
    #[cfg(feature = "diesel_async")]
    PoolBuild(BuildError),
    #[cfg(feature = "diesel")]
    Diesel(diesel::result::Error),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Self::DataNotFound => "Data Not Found".to_string(),
            #[cfg(feature = "diesel_async")]
            Error::Pool(err) => err.to_string(),
            #[cfg(feature = "diesel_async")]
            Error::PoolBuild(err) => err.to_string(),
            #[cfg(feature = "diesel")]
            Error::Diesel(err) => err.to_string(),
        };
        write!(f, "{}", err)
    }
}
#[cfg(feature = "diesel_async")]
impl From<PoolError> for Error {
    fn from(err: PoolError) -> Self {
        Error::Pool(err)
    }
}
#[cfg(feature = "diesel_async")]
impl From<BuildError> for Error {
    fn from(err: BuildError) -> Self {
        Error::PoolBuild(err)
    }
}
#[cfg(feature = "async")]
impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}
