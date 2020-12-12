use actix_files::NamedFile;
use actix_web::{web, Error, HttpResponse};

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

pub async fn index(tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let body = tmpl.render("index.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn favicon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}
