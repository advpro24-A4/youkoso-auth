use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    name: String,
    username: String,
    address: String,
    birth_date: NaiveDate,
    phone_number: String,
}

pub trait ProfileTrait {
    fn new(
        name: String,
        username: String,
        address: String,
        birth_date: NaiveDate,
        phone_number: String,
    ) -> Self;

    fn username(&self) -> &String;

    fn name(&self) -> &String;

    fn address(&self) -> &String;

    fn birth_date(&self) -> &NaiveDate;

    fn phone_number(&self) -> &String;
}

impl ProfileTrait for Profile {
    fn new(
        name: String,
        username: String,
        address: String,
        birth_date: NaiveDate,
        phone_number: String,
    ) -> Self {
        Self {
            name,
            username,
            address,
            birth_date,
            phone_number,
        }
    }

    fn username(&self) -> &String {
        &self.username
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn address(&self) -> &String {
        &self.address
    }

    fn birth_date(&self) -> &NaiveDate {
        &self.birth_date
    }

    fn phone_number(&self) -> &String {
        &self.phone_number
    }
}
