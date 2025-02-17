use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Client error returned from API: {0}")]
    ClientError(String),

    #[error("Response parse error: {0}")]
    ResponseParseError(#[from] serde_json::Error),

    #[error("Missing required 'messages' parameter")]
    MissingMessages,

    #[error("Missing required parameter:{0}")]
    MissingParamsError(String),
    #[error("file does not exist:{0}")]
    PathError(String),

    #[error("Client error:{0}")]
    InnerError(String),
}
