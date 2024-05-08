use nanoid::nanoid;

use crate::models::enumeration::user_type::UserRole;
use crate::models::user::model::profile::Profile;
use crate::models::user::model::user::{User, UserTrait};

pub trait UserBuilderTrait {
    fn with_email(&mut self, email: String) -> &mut Self;
    fn with_user(&mut self, user: User) -> &mut Self;
    fn with_password(&mut self, password: String) -> &mut Self;
    fn with_role(&mut self, role: UserRole) -> &mut Self;
    fn with_profile(&mut self, profile: Profile) -> &mut Self;
    fn with_id(&mut self, id: String) -> &mut Self;
    fn build(&mut self) -> User;
}

#[derive(Debug, Clone)]
pub struct UserBuilder {
    id: Option<String>,
    email: Option<String>,
    password: Option<String>,
    role: Option<UserRole>,
    profile: Option<Profile>,
}

impl Default for UserBuilder {
    fn default() -> Self {
        UserBuilder {
            id: None,
            email: None,
            password: None,
            role: Some(UserRole::User),
            profile: None,
        }
    }
}

impl UserBuilderTrait for UserBuilder {
    fn with_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    fn with_user(&mut self, user: User) -> &mut Self {
        self.email = Some(user.email().to_owned());
        self.password = Some(user.password().to_owned());
        self
    }

    fn with_password(&mut self, password: String) -> &mut Self {
        self.password = Some(password);
        self
    }

    fn with_role(&mut self, role: UserRole) -> &mut Self {
        self.role = Some(role);
        self
    }

    fn with_profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = Some(profile);
        self
    }

    fn with_id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }

    fn build(&mut self) -> User {
        let id = match self.id.clone() {
            Some(id) => id,
            None => nanoid!(),
        };
        User::new(
            id,
            self.email.clone().expect("Email must be filled"),
            self.password.clone().expect("Password must be filled"),
            self.role.clone().unwrap_or(UserRole::User),
            self.profile.clone(),
        )
    }
}
