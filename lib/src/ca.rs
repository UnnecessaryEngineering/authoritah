//! Certificate Authority

use crate::Result;

/// Certificate authority
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertificateAuthority;

/// Certificate authority information
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertificateAuthorityInfo;

impl CertificateAuthority {
    /// Connect to a certificate authority
    pub fn connect<S>(host: S, port: u16) -> Result<Self>
    where
        S: Into<String>,
    {
        Ok(CertificateAuthority {})
    }

    /// Gets basic information regarding the certificate authority
    pub fn info(&self) -> Result<CertificateAuthorityInfo> {
        Ok(CertificateAuthorityInfo {})
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for CertificateAuthorityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "common_name : {}", "CUNT!")
    }
}
