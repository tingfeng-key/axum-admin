use crate::{cache::Info, Result};
use serde::Serialize;

#[cfg(feature = "cache-database")]
pub mod database;
pub mod memory;

// #[async_trait::async_trait]
pub trait Driver {
    /// Storing Items In The Cache
    fn put<T: Serialize + std::marker::Send + std::marker::Sync>(
        &mut self,
        r#type: i32,
        key: &str,
        value: T,
        valid_time_length: Option<i64>,
        attach: Option<String>,
    ) -> impl std::future::Future<Output = Result<Info>> + Send;

    /// Retrieving Items From The Cache
    fn first(
        &self,
        r#type: i32,
        key: &str,
        default: Option<Info>,
    ) -> impl std::future::Future<Output = Result<Option<Info>>> + Send;

    /// Retrieve & Delete
    fn pull(
        &mut self,
        r#type: i32,
        key: &str,
    ) -> impl std::future::Future<Output = Result<Info>> + Send;

    /// clear the entire cache
    fn flush(
        &mut self,
        r#type: Option<i32>,
    ) -> impl std::future::Future<Output = Result<i64>> + Send;
}
