//! Authoritah errors

use crate::ca::CertificateAuthorityError;

/// Error types
#[derive(Debug)]
pub enum Error {
    /// Illegal connection information was supplied
    BadConnectionInfo {
        /// Underlying URL parse error
        err: url::ParseError,
    },
    /// Failure response while communicating with service
    CommunicationFailure {
        /// Underlying HTTP request error
        err: reqwest::Error,
    },
    /// Illegal response payload from service
    BadResponse {
        /// Underlying HTTP parse error
        err: reqwest::Error,
    },
    /// Certificate authority service failure
    CertificateAuthorityFailure {
        /// Underlying certificate authority error
        err: CertificateAuthorityError,
    },
}

impl From<CertificateAuthorityError> for Error {
    fn from(err: CertificateAuthorityError) -> Error {
        Error::CertificateAuthorityFailure { err }
    }
}
