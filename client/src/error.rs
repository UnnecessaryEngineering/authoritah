#[derive(Debug)]
pub(crate) enum Error {
    LogInitializationError { err: log::SetLoggerError },
    CAFailure { err: authoritah::error::Error },
}