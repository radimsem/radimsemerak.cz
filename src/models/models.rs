use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::models::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub content: String,
    pub created_at: NaiveDateTime,
    pub expires: NaiveDateTime
}