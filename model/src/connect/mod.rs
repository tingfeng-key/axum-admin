#[cfg(feature = "diesel_async")]
mod database_diesel_async;

#[cfg(feature = "diesel_async")]
pub use database_diesel_async::{Connect as DbConnect, ConnectPool as DbConnectPool};
