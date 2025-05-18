/*
    Appellation: error <module>
    Contrib: @FL03
*/
#[allow(dead_code)]
/// a type alias for a [`Result`] type that uses the [`AppError`] type as the error type
pub(crate) type AppResult<T = ()> = core::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("")]
    TerminationFailed,
}
