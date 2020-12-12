use std::env;

use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use diesel::{r2d2, PgConnection};
use dotenv::dotenv;
use log::info;
use tera::Tera;

mod controller;
mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server");

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/template/**/*")).unwrap();

    let pool = r2d2::Pool::builder()
        .build(r2d2::ConnectionManager::<PgConnection>::new(
            env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        ))
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .configure(route::route_cfg)
            .service(Files::new("/static", "./static"))
            .data(pool.clone())
            .data(tera.clone())
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
