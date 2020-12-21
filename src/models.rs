use chrono::NaiveDateTime;
use diesel::{prelude::*, PgConnection, QueryResult};

use crate::schema::links::dsl::links as all_links;

#[derive(Queryable, Debug)]
pub struct Link {
    pub id: i32,
    pub shortened: String,
    pub original: String,
    pub created_at: NaiveDateTime,
}

impl Link {
    pub fn get_by_id(conn: &PgConnection, id: &i32) -> QueryResult<Link> {
        all_links.find(id).first(conn)
    }
}
