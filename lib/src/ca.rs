//! Certificate Authority

use serde_derive::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use url::Url;

use crate::{Error, Result};

/// Certificate authority error
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CertificateAuthorityError {
    /// Error code
    pub code: u32,
    /// Error message
    pub message: String,
}

/// Certificate authority information
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CertificateAuthorityInfo {
    /// Certificate authority common name
    #[serde(rename = "commonName")]
    pub common_name: String,
}

impl TryFrom<&CertificateAuthority> for CertificateAuthorityInfo {
    type Error = Error;
    fn try_from(ca: &CertificateAuthority) -> std::result::Result<Self, Self::Error> {
        let endpoint = ca
            .endpoint
            .join("./info")
            .map_err(|err| Error::BadConnectionInfo { err })?;
        let mut result =
            reqwest::get(endpoint).map_err(|err| Error::CommunicationFailure { err })?;
        let status = result.status();
        if status.is_success() {
            result.json().map_err(|err| Error::BadResponse { err })
        } else {
            let server_err: CertificateAuthorityError =
                result.json().map_err(|err| Error::BadResponse { err })?;
            Err(server_err.into())
        }
    }
}

/// Certificate authority
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificateAuthority {
    endpoint: Url,
}

impl CertificateAuthority {
    /// Link to a new certificate authority using protocol version 0
    pub fn v0<S>(host: S, port: u16) -> Result<Self>
    where
        S: Into<String>,
    {
        let url = format!("http://{}:{}/v0/", host.into(), port);
        Ok(CertificateAuthority {
            endpoint: Url::parse(&url).map_err(|err| Error::BadConnectionInfo { err })?,
        })
    }

    /// Gets basic information regarding the certificate authority
    pub fn info(&self) -> Result<CertificateAuthorityInfo> {
        self.try_into()
    }

    /// Initializes a new certificate authority
    pub fn init(&self, info: CertificateAuthorityInfo) -> Result<CertificateAuthorityInfo> {
        Ok(info)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for CertificateAuthorityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "common_name : {}", self.common_name)
    }
}
