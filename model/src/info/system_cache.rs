use serde::Serialize;
use utils::datetime::now_time;

#[derive(Debug, Clone, Serialize)]
pub struct Info {
    pub key: String,
    pub r#type: i32,
    pub value: String,
    pub attach: Option<String>,
    pub valid_time_length: Option<i64>,
    pub create_time: i64,
}

impl Info {
    pub fn is_valid(self) -> bool {
        if let Some(valid_time_length) = self.valid_time_length {
            return (self.create_time + valid_time_length) > now_time().timestamp();
        }
        false
    }

    pub fn valid_timestamp(self) -> Option<i64> {
        self.valid_time_length.map(|x| self.create_time + x)
    }

    pub fn value<T: serde::de::DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_str::<T>(&self.value)
    }
}
