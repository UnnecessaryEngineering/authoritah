#[derive(Debug)]
pub(crate) enum Error {
    NoCommandProvided,
    CommandLineError { err: clap::Error },
    LogInitializationError { err: log::SetLoggerError },
    RequestFailed { err: authoritah::error::Error },
}

impl From<clap::Error> for Error {
    fn from(clap_err: clap::Error) -> Error {
        Error::CommandLineError { err: clap_err }
    }
}
