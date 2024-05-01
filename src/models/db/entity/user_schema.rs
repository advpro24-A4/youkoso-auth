use chrono::NaiveDate;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::models::enumeration::user_type::UserRole;

#[derive(Queryable, Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name=crate::models::db::schema::users)]
pub struct UserInsertSchema {
    pub id: String,
    pub email: String,
    pub password: String,
    pub role: Option<UserRole>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub address: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
}
