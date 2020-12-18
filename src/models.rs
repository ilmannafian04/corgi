use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Link {
    pub id: String,
    pub original: String,
    pub created_date: NaiveDateTime,
}
