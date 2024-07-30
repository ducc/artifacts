use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("api error: {:?}", ._0)]
    APIError(#[from] crate::response::ErrorData),
    #[error("invalid api response")]
    InvalidAPIResponse,
    #[error("serde json error: {0}")]
    SerdeJSON(#[from] serde_json::Error),
    #[error("character in cooldown")]
    Cooldown,
}
