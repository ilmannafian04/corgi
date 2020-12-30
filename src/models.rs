use chrono::NaiveDateTime;
use diesel::{insert_into, prelude::*, PgConnection, QueryResult};

use crate::schema::links::{self, dsl::links as all_links, dsl::shortened};

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
}

impl Link {
    pub fn insert_link(conn: &PgConnection, new_link: &NewLink) -> QueryResult<Link> {
        insert_into(links::table).values(new_link).get_result(conn)
    }

    pub fn get_link_by_shortened(conn: &PgConnection, shortened_str: &String) -> QueryResult<Link> {
        all_links
            .filter(shortened.eq(shortened_str))
            .first::<Link>(conn)
    }
}
