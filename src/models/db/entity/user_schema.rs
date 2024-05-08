use crate::models::{db::schema::users, enumeration::user_type::UserRole};
use chrono::NaiveDate;
use diesel::{deserialize::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserSelectSchema {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub address: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub role: Option<UserRole>,
}
