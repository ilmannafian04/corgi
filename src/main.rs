use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/api/ping",
            web::get().to(|| web::HttpResponse::Ok().body("pong")),
        )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
