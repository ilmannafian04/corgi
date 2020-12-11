use actix_web::web;

use crate::api;

pub fn route_cfg(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/")
            .route(web::get().to(api::index))
        )
        .service(web::resource("/favicon.ico")
            .route(web::get().to(api::favicon))
        );
}
