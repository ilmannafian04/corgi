use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;

mod api;
mod route;

#[get("/api/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
