use actix_web::{web, App, Error as ActixError, HttpResponse, HttpServer};
use authoritah::ca::{CertificateAuthorityError, CertificateAuthorityInfo};
use diesel::prelude::*;
use futures::Future;
use log::*;

use crate::{db::Pool, error::Error, Result};

fn info(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = ActixError> {
    use crate::db::model::CertificateAuthority;
    use crate::db::schema::certificate_authority::dsl::*;
    debug!("received authority information request");
    // TODO don't unwrap blindly here, you fool!
    web::block(move || certificate_authority.load::<CertificateAuthority>(&pool.get().unwrap()))
        .then(|result| match result {
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
        })
}

pub(super) fn run(db: Pool) -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .route("/v0/info", web::get().to_async(info))
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to port 8000")
    .run()
    .map_err(|err| Error::ApiError { err })
}
