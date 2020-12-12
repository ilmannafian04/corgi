use std::env;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::r2d2;
use diesel::PgConnection;
use dotenv::dotenv;
use handlebars::Handlebars;
use log::info;

mod controller;
mod route;

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

    let pool = r2d2::Pool::builder().build(r2d2::ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL not set"),
    )).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .configure(route::route_cfg)
            .service(Files::new("/static", "./static"))
            .app_data(hb_ref.clone())
            .data(pool.clone())
            .wrap(Logger::default())
    })
    .bind(format!(
        "{}:{}",
        env::var("HOST").expect("HOST not set"),
        env::var("PORT").expect("PORT not set")
    ))?
    .run()
    .await
}
