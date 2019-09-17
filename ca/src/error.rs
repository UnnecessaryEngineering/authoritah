#[derive(Debug)]
pub enum Error {
    EllipticCurveFailure { cause: cryptonomicon::elliptic::Error },
}

impl From<cryptonomicon::elliptic::Error> for Error {
    fn from(cause: cryptonomicon::elliptic::Error) -> Self {
        Self::EllipticCurveFailure { cause }
    }
}