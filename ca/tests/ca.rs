#[test]
fn initialize() {
    use authoritah_ca::prelude::*;
    let subject = X500Subject::new("Test CA");
    let ca = CertificateAuthority::new(subject);
}