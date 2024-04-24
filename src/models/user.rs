use bcrypt::{hash_with_result, Version, DEFAULT_COST};
use diesel::sql_types::Date;
use nanoid::nanoid;

use crate::models::enumeration::user_type::UserRole;

pub struct User {
    id: String,
    email: String,
    name: Option<String>,
    username: Option<String>,
    password: String,
    role: UserRole,
    address: Option<String>,
    birth_date: Option<Date>,
    phone_number: Option<String>,
}

trait NewTrait {
    fn new(&self, email: String, password: String, role: UserRole) -> User;
    fn encrypt_password(&self) -> String;
}

impl NewTrait for User {
    fn new(&self, email: String, password: String, role: UserRole) -> User {
        User {
            id: nanoid!(),
            email,
            name: None,
            username: None,
            password,
            role,
            address: None,
            birth_date: None,
            phone_number: None,
        }
    }

    fn encrypt_password(&self) -> String {
        let hashed_password = hash_with_result(&self.password, DEFAULT_COST).unwrap();
        hashed_password.format_for_version(Version::TwoB)
    }
}
