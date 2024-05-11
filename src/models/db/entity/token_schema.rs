use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};

use crate::models::db::schema::tokens;

#[derive(Queryable, Selectable, Debug, Clone, Insertable)]
#[diesel(table_name = tokens)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
    pub status: String,
}
