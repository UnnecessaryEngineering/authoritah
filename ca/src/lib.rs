use chrono::prelude::*;
use cryptonomicon::asymmetric::{KeyPair, PublicKey};
use cryptonomicon::elliptic::EllipticCurve;
use cryptonomicon::pkcs10::PKCS10;
use cryptonomicon::x500::X500Subject;
use cryptonomicon::x509::X509Certificate;

pub mod prelude {
    pub use crate::error::Error;
    pub use crate::CertificateAuthority;
    pub use cryptonomicon::x500::X500Subject;
}

pub mod error;
use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct CertificateAuthority {
    keypair: Box<dyn KeyPair>,
    subject: X500Subject,
}

impl std::fmt::Debug for CertificateAuthority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CertificateAuthority {{ subject: {:?} }}", self.subject)
    }
}

impl CertificateAuthority {
    pub fn new(subject: X500Subject) -> Result<Self> {
        let keypair = EllipticCurve::Curve25519.generate_keypair()?;
        Ok(Self {
            keypair,
            subject,
        })
    }

    pub fn sign<'a>(
        &self,
        pkcs10: &'a PKCS10,
        not_before: DateTime<Utc>,
        not_after: DateTime<Utc>,
    ) -> X509Certificate<'a> {
        let serial = 1;
        let subject = pkcs10.subject.clone();
        let public_key = pkcs10.public_key.clone();
        let issuer = self.subject.clone();
        X509Certificate::new(serial, subject, *public_key, issuer, not_before, not_after)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Duration;

    #[test]
    fn certificate_authority_new() {
        let subject = X500Subject::new("Test CN");
        let actual = CertificateAuthority::new(subject.clone()).unwrap();
        assert_eq!(subject, actual.subject);
    }

    #[test]
    fn certificate_authority_issue() {
        let keypair = EllipticCurve::Curve25519.generate_keypair().unwrap();
        let public_key = keypair.public_key();

        let now = Utc::now();
        let until = now + Duration::days(365);

        let ca_subject = X500Subject::new("Test CA");
        let ca = CertificateAuthority::new(ca_subject.clone()).unwrap();
        let subject = X500Subject::new("Test User");
        let pkcs10 = PKCS10::new(subject.clone(), &*public_key);
        let cert = ca.sign(&pkcs10, now, until);

        assert_eq!(ca_subject, cert.issuer);
        assert_eq!(subject, cert.subject);
        assert_eq!(now, cert.not_before);
        assert_eq!(until, cert.not_after);
    }
}
