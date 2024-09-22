use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

pub mod tetification;
pub use tetification::*;
pub mod tet;
pub use tet::*;
pub mod user;
pub use user::*;

#[derive(Serialize, Deserialize, Debug, Error)]
#[serde(rename_all = "camelCase")]
#[error("Tetter error: {error}: {message}")]
pub struct TetterError {
    pub error: String,
    pub message: String,
    pub status_code: i32,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Tetter error: {0}")]
    Tetter(TetterError),
    #[error("Serde error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub type ApiResult<T> = Result<T, Error>;

fn deserialize_ts_from_opt_i64<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_i64 = Option::deserialize(deserializer)?;
    Ok(opt_i64
        .map(chrono::DateTime::from_timestamp_millis)
        .unwrap_or_default())
}

fn serialize_ts_to_opt_i64<S>(
    ts: &Option<chrono::DateTime<chrono::Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ts.map(|ts| ts.timestamp_millis()).serialize(serializer)
}
