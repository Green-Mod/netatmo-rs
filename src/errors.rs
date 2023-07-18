use thiserror::Error;

/// The error kind for errors that get returned in the crate
#[derive(Eq, PartialEq, Debug, Error, Clone)]
pub enum NetatmoError {
    #[error("Failed to deserialize JSON")]
    JsonDeserializationFailed,

    #[error("Failed to send request")]
    FailedToSendRequest,

    #[error("Failed to read response")]
    FailedToReadResponse,

    #[error("Failed to authenticate")]
    AuthenticationFailed,

    #[error("API call '{name}' failed with code {code} because {msg}")]
    ApiCallFailed { name: String, code: isize, msg: String },

    #[error("API call '{name}' failed for unknown reason with status code {status_code}")]
    UnknownApiCallFailure { name: String, status_code: u16 },
}

pub type Result<T> = ::std::result::Result<T, NetatmoError>;
