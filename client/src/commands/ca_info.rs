use authoritah::prelude::*;
use log::*;

use crate::error::Error;
use crate::Result;

pub(crate) fn exec(config: crate::config::Config) -> Result<()> {
    debug!("executing ca_info command");

    let ca = CertificateAuthority::connect(config.ca.host, config.ca.port)
        .map_err(|err| Error::CAFailure { err })?;

    println!("{}", ca.info().map_err(|err| Error::CAFailure { err })?);

    Ok(())
}
