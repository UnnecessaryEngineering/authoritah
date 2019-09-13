//! Authoritah is a lightweight [PKI] platform to create and administer private
//! certificate authorities. This crate is a rust-native library for
//! interacting with Authoritah services.
//!
//! [PKI]: https://wikipedia.org/wiki/Public_key_infrastructure
#![doc(
    html_logo_url = "https://labs.unnecessary.engineering/logo.png",
    html_favicon_url = "https://labs.unnecessary.engineering/favicon.ico",
    issue_tracker_base_url = "https://github.com/UnnecessaryEngineering/authoritah/issues/"
)]
#![deny(missing_docs, unused_imports, missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod ca;

pub mod error;
use error::Error;

/// Result type for public methods
pub type Result<T> = core::result::Result<T, Error>;

/// Convenience module for accessing standard public interfaces (`use authoritah::prelude::*`)
pub mod prelude {
    pub use crate::ca::CertificateAuthority;
    pub use crate::ca::CertificateAuthorityInfo;
}
