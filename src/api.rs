use actix_files::NamedFile;
use actix_web::{get, web, Error, HttpResponse};
use handlebars::Handlebars;

#[get("/api/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    HttpResponse::Ok().body(hb.render("index", &()).unwrap())
}

#[get("/favicon.ico")]
pub async fn favicon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}
