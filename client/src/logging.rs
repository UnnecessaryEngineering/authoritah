use log::*;

use crate::config::LoggingConfig;
use crate::error::Error;
use crate::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub(crate) fn initialize(logging_config: LoggingConfig) -> Result<()> {
    let level = match logging_config.level {
        Some(Level::Trace) => log::LevelFilter::Trace,
        Some(Level::Debug) => log::LevelFilter::Debug,
        Some(Level::Info) => log::LevelFilter::Info,
        Some(Level::Warn) => log::LevelFilter::Warn,
        Some(Level::Error) => log::LevelFilter::Error,
        None => log::LevelFilter::Off,
    };
    pretty_env_logger::formatted_timed_builder()
        .filter_level(level)
        .try_init()
        .map_err(|err| Error::LogInitializationError { err })?;
    trace!("logging initialized");
    Ok(())
}
