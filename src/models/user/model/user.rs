use bcrypt::{hash_with_result, verify, Version, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use crate::models::enumeration::user_type::UserRole;
use crate::models::user::model::profile::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
    role: UserRole,
    profile: Option<Profile>,
}

pub trait UserTrait {
    fn new(
        id: String,
        email: String,
        password: String,
        role: UserRole,
        profile: Option<Profile>,
    ) -> Self;
    fn id(&self) -> &String;
    fn email(&self) -> &String;
    fn password(&self) -> &String;
    fn role(&self) -> &UserRole;
    fn profile(&self) -> &Option<Profile>;
    fn encrypt_password(&self) -> String;
    fn verify_password(&self, hashed_password: String) -> bool;
}

impl UserTrait for User {
    fn new(
        id: String,
        email: String,
        password: String,
        role: UserRole,
        profile: Option<Profile>,
    ) -> Self {
        Self {
            id: id,
            email,
            password,
            role,
            profile,
        }
    }

    fn id(&self) -> &String {
        &self.id
    }

    fn email(&self) -> &String {
        &self.email
    }

    fn password(&self) -> &String {
        &self.password
    }

    fn role(&self) -> &UserRole {
        &self.role
    }

    fn profile(&self) -> &Option<Profile> {
        &self.profile
    }
    fn encrypt_password(&self) -> String {
        let hashed_password = hash_with_result(&self.password, DEFAULT_COST).unwrap();
        hashed_password.format_for_version(Version::TwoB)
    }

    fn verify_password(&self, hashed_password: String) -> bool {
        let verified = verify(self.password(), hashed_password.as_str());
        match verified {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
