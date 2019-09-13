use crate::{Command, Result};
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Config {
    pub(crate) ca: CAConfig,
    pub(crate) command: Command,
    pub(crate) logging: LoggingConfig,
}

impl TryFrom<&clap::ArgMatches<'_>> for Config {
    type Error = crate::Error;
    fn try_from(matches: &clap::ArgMatches) -> std::result::Result<Config, Self::Error> {
        let ca = CAConfig::default();
        let (command_name, command_args) = matches.subcommand();
        if command_args.is_none() {
            return Err(Self::Error::NoCommandProvided);
        }
        let command = resolve_command(command_name, command_args.unwrap());
        let logging = LoggingConfig {
            level: resolve_logging_level(matches.occurrences_of("verbose")),
        };
        Ok(Config {
            ca,
            command,
            logging,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CAConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
}

impl Default for CAConfig {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 8000,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct LoggingConfig {
    pub(crate) level: Option<crate::logging::Level>,
}

pub(crate) fn load() -> Result<Config> {
    let yaml = clap::load_yaml!("cli.yml");
    let mut app = clap::App::from_yaml(yaml);
    let app2 = app.clone();
    match (&app2.get_matches()).try_into() {
        Ok(config) => Ok(config),
        Err(err) => {
            eprintln!("ERR: {:?}", err);
            app.print_help()?;
            std::process::exit(2);
        }
    }
}

fn resolve_command(name: &str, matches: &clap::ArgMatches) -> Command {
    match name {
        "ca-cert" => Command::CACert,
        "ca-init" => Command::CAInit {
            common_name: matches.value_of("common_name").unwrap().into(),
        },
        _ => Command::CAInfo,
    }
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

    // TODO: make this work with argmatches from clap
    // #[test]
    // fn resolve_command_ca_info() {
    //     assert_eq!(resolve_command(None), crate::Command::CAInfo);
    //     assert_eq!(resolve_command(Some("ca-info")), crate::Command::CAInfo);
    // }

    // TODO: make this work with argmatches from clap
    // #[test]
    // fn resolve_command_ca_init() {
    //     assert_eq!(resolve_command(Some("ca-init")), crate::Command::CAInit);
    // }

    // TODO: make this work with argmatches from clap
    // #[test]
    // fn resolve_command_ca_cert() {
    //     assert_eq!(resolve_command(Some("ca-cert")), crate::Command::CACert);
    // }

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
