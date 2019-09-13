use log::*;

use crate::Result;

pub(crate) fn exec(_config: &crate::config::Config) -> Result<()> {
    debug!("executing ca_cert command");
    Ok(())
}
