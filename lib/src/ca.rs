//! Certificate Authority

use serde_derive::{Deserialize, Serialize};

use crate::Result;

/// Certificate authority
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificateAuthority {
    common_name: String,
}

/// Certificate authority information
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CertificateAuthorityInfo {
    /// Certificate authority common name
    #[serde(rename="commonName")]
    pub common_name: String,
}

impl From<&CertificateAuthority> for CertificateAuthorityInfo {
    fn from(ca: &CertificateAuthority) -> CertificateAuthorityInfo {
        CertificateAuthorityInfo {
            common_name: ca.common_name.clone(),
        }
    }
}

impl CertificateAuthority {
    /// Connect to a certificate authority
    pub fn connect<S>(host: S, port: u16) -> Result<Self>
    where
        S: Into<String>,
    {
        Ok(CertificateAuthority {
            common_name: "Testing".into(),
        })
    }

    /// Gets basic information regarding the certificate authority
    pub fn info(&self) -> Result<CertificateAuthorityInfo> {
        Ok(self.into())
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for CertificateAuthorityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "common_name : {}", self.common_name)
    }
}
