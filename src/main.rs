#[macro_use]
extern crate log;

use std::env;

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use handlebars::Handlebars;

mod api;
mod route;

#[get("/api/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./template")
        .unwrap();
    let hb_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .service(ping)
            .configure(route::route_cfg)
            .service(Files::new("/static", "./static"))
            .app_data(hb_ref.clone())
    })
    .bind(format!("{}:{}", env::var("HOST").expect("HOST not set"), env::var("PORT").expect("PORT not set")))?
    .run()
    .await
}
