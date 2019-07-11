#[derive(Debug)]
pub(crate) enum Error {
    LogInitializationError { err: log::SetLoggerError },
    RequestFailed { err: authoritah::error::Error },
}