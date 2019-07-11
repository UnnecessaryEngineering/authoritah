use actix_web::{HttpServer, HttpRequest, Responder, App, web};

use crate::{error::Error, Result};

fn info(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub(super) fn run() -> Result<()> {
        HttpServer::new(|| {
        App::new()
            .route("/info", web::get().to(info))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run().map_err(|err| Error::ApiError { err })
}