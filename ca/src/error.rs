#[derive(Debug)]
pub(crate) enum Error {
    LogInitializationError { err: log::SetLoggerError },
    ApiError { err: std::io::Error },
    DatabaseConnectionFailed { err: r2d2::Error },
}
