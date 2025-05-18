/*
    Appellation: error <module>
    Contrib: @FL03
*/
#[allow(dead_code)]
/// a type alias for a [`Result`] type that uses the [`Error`] type as the error type
pub(crate) type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug, strum::EnumIs, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    AppError(#[from] crate::app::AppError),
    #[error("[Unknown Error] {0}")]
    UnknownError(String),
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "cf")]
    #[error(transparent)]
    WorkerError(#[from] worker::Error),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::UnknownError(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::UnknownError(err.to_string())
    }
}
