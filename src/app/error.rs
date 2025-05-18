/*
    Appellation: error <module>
    Contrib: @FL03
*/
#[allow(dead_code)]
/// a type alias for a [`Result`] type that uses the [`AppError`] type as the error type
pub(crate) type AppResult<T = ()> = core::result::Result<T, AppError>;

#[derive(Debug, strum::EnumIs, thiserror::Error)]
pub enum AppError {
    #[error("Shutdown Failed: {0}")]
    ShutdownFailed(String),
    #[error("Shutdown Timeout: {0}")]
    ShutdownTimeout(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
