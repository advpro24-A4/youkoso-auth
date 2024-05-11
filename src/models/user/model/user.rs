use argon2::password_hash::PasswordHasher;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
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
            id,
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
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(self.password.as_ref(), &salt)
            .unwrap()
            .to_string();
        hashed_password
    }
}
