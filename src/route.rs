use actix_web::web;

use crate::controller as c;

pub fn route_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/ping", web::get().to(c::ping)))
        .service(
            web::scope("")
                .route("/shorten", web::post().to(c::shorten))
                .route("/favicon.ico", web::get().to(c::favicon))
                .route("/", web::get().to(c::index)),
        );
}
