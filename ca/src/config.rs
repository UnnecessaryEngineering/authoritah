use crate::Result;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Config {
    pub(crate) logging: LoggingConfig,
    pub(crate) database: DatabaseConfig,
}

impl From<clap::ArgMatches<'_>> for Config {
    fn from(matches: clap::ArgMatches) -> Config {
        let logging = LoggingConfig {
            level: resolve_logging_level(matches.occurrences_of("verbose")),
        };
        let database = DatabaseConfig {
            url: matches
                .value_of("database_url")
                .unwrap_or("mysql://localhost/authoritah_ca")
                .into(),
        };
        Config { logging, database }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct LoggingConfig {
    pub(crate) level: Option<crate::logging::Level>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DatabaseConfig {
    pub(crate) url: String,
}

pub(crate) fn load() -> Result<Config> {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    Ok(matches.into())
}

fn resolve_logging_level(verbosity: u64) -> Option<crate::logging::Level> {
    use crate::logging::Level::*;
    if verbosity < 1 {
        Some(Error)
    } else if verbosity == 1 {
        Some(Warn)
    } else if verbosity == 2 {
        Some(Info)
    } else if verbosity == 3 {
        Some(Debug)
    } else {
        Some(Trace)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resolve_logging_level_error() {
        assert_eq!(resolve_logging_level(0), Some(crate::logging::Level::Error));
    }

    #[test]
    fn resolve_logging_level_warn() {
        assert_eq!(resolve_logging_level(1), Some(crate::logging::Level::Warn));
    }

    #[test]
    fn resolve_logging_level_info() {
        assert_eq!(resolve_logging_level(2), Some(crate::logging::Level::Info));
    }

    #[test]
    fn resolve_logging_level_debug() {
        assert_eq!(resolve_logging_level(3), Some(crate::logging::Level::Debug));
    }

    #[test]
    fn resolve_logging_level_trace() {
        assert_eq!(resolve_logging_level(4), Some(crate::logging::Level::Trace));
    }

    #[test]
    #[ignore]
    fn resolve_logging_level_trace_all() {
        for i in 4..=std::u64::MAX {
            assert_eq!(resolve_logging_level(i), Some(crate::logging::Level::Trace));
        }
    }
}
