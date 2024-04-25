use bcrypt::{hash_with_result, Version, DEFAULT_COST};
use chrono::NaiveDate;
use nanoid::nanoid;

use crate::models::enumeration::user_type::UserRole;

#[derive(Debug, Clone)]
pub struct User {
    id: String,
    email: String,
    name: Option<String>,
    username: Option<String>,
    password: String,
    role: UserRole,
    address: Option<String>,
    birth_date: Option<NaiveDate>,
    phone_number: Option<String>,
}

pub trait UserTrait {
    fn new(email: String, password: String, role: UserRole) -> User;
    fn get_id(&self) -> String;
    fn get_email(&self) -> String;
    fn get_username(&self) -> String;
    fn get_name(&self) -> String;
    fn get_password(&self) -> String;
    fn get_role(&self) -> String;
    fn get_address(&self) -> String;
    fn get_birth_date(&self) -> NaiveDate;
    fn get_phone_number(&self) -> String;
    fn set_name(&self, name: String);
    fn set_username(&self, username: String);
    fn set_address(&self, address: String);
    fn set_birth_date(&self, date: NaiveDate);
    fn set_phone_number(&self, phone_number: String);
    fn encrypt_password(&self) -> String;
}

impl UserTrait for User {
    fn new(email: String, password: String, role: UserRole) -> User {
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

    fn get_id(&self) -> String {
        unimplemented!()
    }

    fn get_email(&self) -> String {
        unimplemented!()
    }

    fn get_username(&self) -> String {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        unimplemented!()
    }

    fn get_password(&self) -> String {
        unimplemented!()
    }

    fn get_role(&self) -> String {
        unimplemented!()
    }

    fn get_address(&self) -> String {
        unimplemented!()
    }

    fn get_birth_date(&self) -> NaiveDate {
        unimplemented!()
    }

    fn get_phone_number(&self) -> String {
        unimplemented!()
    }

    fn set_name(&self, name: String) {
        unimplemented!()
    }

    fn set_username(&self, username: String) {
        unimplemented!()
    }

    fn set_address(&self, address: String) {
        unimplemented!()
    }

    fn set_birth_date(&self, date: NaiveDate) {
        unimplemented!()
    }

    fn set_phone_number(&self, phone_number: String) {
        unimplemented!()
    }

    fn encrypt_password(&self) -> String {
        let hashed_password = hash_with_result(&self.password, DEFAULT_COST).unwrap();
        hashed_password.format_for_version(Version::TwoB)
    }
}
