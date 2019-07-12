//! Certificate authority database connection and model

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::config::DatabaseConfig;
use crate::{error::Error, Result};

pub(crate) mod model;
pub(crate) mod schema;

pub(crate) type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub(crate) fn connect(config: DatabaseConfig) -> Result<Pool> {
    let manager = ConnectionManager::<MysqlConnection>::new(config.url);
    r2d2::Pool::builder()
        .build(manager)
        .map_err(|err| Error::DatabaseConnectionFailed { err })
}
