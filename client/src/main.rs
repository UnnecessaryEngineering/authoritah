//! Authoritah is a lightweight [PKI] platform to create and administer private
//! certificate authorities. This crate includes a command line client for
//! interacting with Authoritah services.
//!
//! [PKI]: https://wikipedia.org/wiki/Public_key_infrastructure
#![doc(
    html_logo_url = "https://labs.unnecessary.engineering/logo.png",
    html_favicon_url = "https://labs.unnecessary.engineering/favicon.ico",
    issue_tracker_base_url = "https://github.com/UnnecessaryEngineering/authoritah/issues/"
)]
#![deny(missing_docs, unused_imports, missing_debug_implementations)]

mod commands;

mod config;

mod error;
use error::Error;

mod logging;

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Command {
    CACert,
    CAInfo,
}

fn main() -> Result<()> {
    let config = config::load()?;
    
    logging::initialize(config.logging)?;

    match config.command {
        Command::CACert => commands::ca_cert::exec(config)?,
        Command::CAInfo => commands::ca_info::exec(config)?,
    };

    Ok(())
}
