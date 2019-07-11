//! Authoritah is a lightweight [PKI] platform to create and administer private
//! certificate authorities. This crate includes the certificate authority
//! server dameon.
//!
//! [PKI]: https://wikipedia.org/wiki/Public_key_infrastructure
#![doc(
    html_logo_url = "https://labs.unnecessary.engineering/logo.png",
    html_favicon_url = "https://labs.unnecessary.engineering/favicon.ico",
    issue_tracker_base_url = "https://github.com/UnnecessaryEngineering/authoritah/issues/"
)]
#![deny(missing_docs, unused_imports, missing_debug_implementations)]

#[macro_use]
extern crate diesel;

mod api;

mod config;

mod db;

mod error;
use error::Error;

mod logging;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let config = config::load()?;

    logging::initialize(config.logging)?;

    let db = db::connect(config.database)?;

    api::run(db)?;

    Ok(())
}
