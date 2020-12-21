use chrono::NaiveDateTime;
use diesel::{debug_query, prelude::*, PgConnection, QueryResult};

use crate::schema::links::{self, dsl::links as all_links};

#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub shortened: String,
    pub original: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink {
    pub shortened: String,
    pub original: String,
    pub created_at: NaiveDateTime,
}

impl Link {
    pub fn get_link_by_id(conn: &PgConnection, id: &i32) -> QueryResult<Link> {
        all_links.find(id).first(conn)
    }

    pub fn insert_link(conn: &PgConnection, new_link: &NewLink) -> QueryResult<Link> {
        let q = diesel::insert_into(links::table).values(new_link);
        println!("{}", debug_query(&q));
        q.get_result(conn)
    }
}
