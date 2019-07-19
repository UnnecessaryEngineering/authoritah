use actix_web::{web, App, Error as ActixError, HttpResponse, HttpServer};
use authoritah::ca::{CertificateAuthorityError, CertificateAuthorityInfo};
use diesel::prelude::*;
use futures::Future;
use log::*;

use crate::{db::Pool, error::Error, Result};

fn get_database_connection(pool: web::Data<Pool>) -> impl Future<Item = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>, Error = HttpResponse> {
    web::block(move || pool.get()).then(|connection_result| match connection_result {
        Ok(connection) => Ok(connection),
        Err(err) => {
            error!("failed to obtain database connection from pool: {}", err);
            Err(
                    HttpResponse::InternalServerError().json(CertificateAuthorityError {
                        code: 1,
                        message: "internal server error".into(),
                    }),
                )
        }
    })
}

fn info(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = ActixError> {
    use crate::db::model::CertificateAuthority;
    use crate::db::schema::certificate_authority::dsl::*;
    debug!("received certificate authority information request");
    get_database_connection(pool).then(|result| match result {
        Ok(connection) => {
            match certificate_authority.load::<CertificateAuthority>(&connection) {
                Ok(ca) => {
                    if ca.len() < 1 {
                        warn!("information request failed due to no certificate authority configuration in database");
                        Ok(HttpResponse::ServiceUnavailable().json(CertificateAuthorityError {
                            code: 2,
                            message: "certificate authority not configured".into(),
                        }))
                    } else {
                        let info: CertificateAuthorityInfo = (&ca[0]).into();
                        debug!("information request successful: {:?}", &info);
                        Ok(HttpResponse::Ok().json(info))
                    }
                }
                Err(err) => {
                    error!("failed to query database: {}", err);
                    Ok(
                        HttpResponse::InternalServerError().json(CertificateAuthorityError {
                            code: 1,
                            message: "internal server error".into(),
                        }),
                    )
                }
            }
        }
        Err(response) => Ok(response)
    })
}

fn init(
    profile: web::Json<CertificateAuthorityInfo>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ActixError> {
    use crate::db::model::CertificateAuthority;
    use crate::db::schema::certificate_authority::dsl::*;
    debug!("received authority initialization request: {:#?}", &profile);
    get_database_connection(pool).then(|result| match result {
        Ok(connection) => {
            match certificate_authority.load::<CertificateAuthority>(&connection) {
                Ok(ca) => {
                    if ca.len() > 0 {
                        warn!("attempt to initialize certificate authority rejected: authority already initialized");
                        Ok(HttpResponse::Conflict().json(CertificateAuthorityError {
                            code: 3,
                            message: "certificate authority already initialized".into(),
                        }))
                    } else {
                        let info: CertificateAuthorityInfo = CertificateAuthorityInfo {
                            common_name: "TESTING".into(),
                        };
                        Ok(HttpResponse::Ok().json(info))
                    }
                }
                Err(err) => {
                    error!("failed to query database: {}", err);
                    Ok(
                    HttpResponse::InternalServerError().json(CertificateAuthorityError {
                        code: 1,
                        message: "internal server error".into(),
                    }),
                )
                }
            }
        }
        Err(response) => Ok(response)
    })
}

pub(super) fn run(db: Pool) -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .route("/v0/info", web::get().to_async(info))
            .route("/v0/init", web::post().to_async(init))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .map_err(|err| Error::ApiError { err })
}
