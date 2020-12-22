use actix_files::NamedFile;
use actix_web::{web, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use rand::{distributions::Alphanumeric, prelude::*, thread_rng};
use serde::Deserialize;

use crate::models::{Link, NewLink};

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

pub async fn index(tmpl: web::Data<tera::Tera>) -> HttpResponse {
    let body = tmpl.render("index.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn favicon() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}

#[derive(Deserialize)]
pub struct ShortenParam {
    link: String,
}

pub async fn shorten(
    pool: web::Data<DbPool>,
    param: web::Form<ShortenParam>,
    tmpl: web::Data<tera::Tera>,
) -> HttpResponse {
    let conn = pool.get().expect("cannot get the DB pool");
    let new_link = NewLink {
        shortened: thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect(),
        original: param.link.to_owned(),
    };
    let link = web::block(move || Link::insert_link(&conn, &new_link)).await;
    match link {
        Ok(link) => {
            let body = tmpl.render("shorten.html", &tera::Context::new()).unwrap();
            HttpResponse::Ok().body(body)
        }
        Err(e) => HttpResponse::NotFound().body(format!("{}", e)),
    }
}
