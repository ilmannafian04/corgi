use chrono::NaiveDateTime;

use crate::schema::links;

#[derive(Insertable, Queryable)]
pub struct Link {
    pub id: String,
    pub original: String,
    pub created_at: NaiveDateTime,
}
