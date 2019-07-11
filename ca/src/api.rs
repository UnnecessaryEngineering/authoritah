use actix_web::{web, App,Error as ActixError, HttpRequest, HttpResponse, HttpServer};
use authoritah::ca::CertificateAuthorityInfo;
use futures::{future, Future};
use log::*;

use crate::{error::Error, Result};

fn info(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = ActixError> {
    debug!("Received info request {:#?}", req);
    let info = CertificateAuthorityInfo {
        common_name: "Wazzzzup".into(),
    };
    debug!("Returning info {:#?}", &info);
    future::ok(HttpResponse::Ok().json(info))
}

pub(super) fn run() -> Result<()> {
    HttpServer::new(|| App::new().route("/info", web::get().to_async(info)))
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run()
        .map_err(|err| Error::ApiError { err })
}
