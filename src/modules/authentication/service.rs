use deadpool_diesel::postgres::Pool;

use crate::models::user::{
    builder::user_builder::{UserBuilder, UserBuilderTrait},
    director::{UserDirector, UserDirectorTrait},
    model::user::User,
};

use super::repository::{AuthenticationRepository, AuthenticationRepositoryTrait};

pub trait AuthenticationServiceTrait {
    fn login(user: User) -> User;
    async fn register(
        &self,
        email: String,
        password: String,
        pool: &Pool,
    ) -> Result<User, Box<dyn std::error::Error>>;
    fn verify_password(password: String) -> bool;
}

trait PrivateAuthenticationServiceTrait {
    fn repository(&self) -> &AuthenticationRepository;
}

#[derive(Default)]
pub struct AuthenticationService {
    repository: AuthenticationRepository,
}

impl PrivateAuthenticationServiceTrait for AuthenticationService {
    fn repository(&self) -> &AuthenticationRepository {
        &self.repository
    }
}

impl AuthenticationServiceTrait for AuthenticationService {
    fn login(user: User) -> User {
        unimplemented!()
    }
    async fn register(
        &self,
        email: String,
        password: String,
        pool: &Pool,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let repository = self.repository();
        let mut builder = UserBuilder::default();
        _ = UserDirector::construct_register_customer_user(&mut builder, email, password);
        let registered_user: User = builder.build();
        _ = repository.create_user(&registered_user, pool);
        Ok(registered_user)
    }

    fn verify_password(password: String) -> bool {
        unimplemented!()
    }
}
