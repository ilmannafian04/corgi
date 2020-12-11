use actix_files::NamedFile;
use actix_web::{web, Error, HttpResponse};
use handlebars::Handlebars;

pub async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    HttpResponse::Ok().body(hb.render("index", &()).unwrap())
}

pub async fn favicon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}
