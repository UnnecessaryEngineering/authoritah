use authoritah::ca::CertificateAuthorityInfo;
use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct CertificateAuthority {
    pub id: u32,
    pub common_name: String,
}

impl Into<CertificateAuthorityInfo> for &CertificateAuthority {
    fn into(self) -> CertificateAuthorityInfo {
        CertificateAuthorityInfo {
            common_name: self.common_name.clone(),
        }
    }
}
