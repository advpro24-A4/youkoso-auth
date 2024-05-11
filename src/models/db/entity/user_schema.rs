use crate::models::{db::schema::users, enumeration::user_type::UserRole};
use chrono::NaiveDate;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct UserSelectSchema {
    pub id: String,
    pub role: UserRole,
    pub email: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub address: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
}
