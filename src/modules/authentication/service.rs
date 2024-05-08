use axum::{http::StatusCode, Json};
use bcrypt::{verify, BcryptError};
use deadpool_diesel::postgres::Pool;

use crate::{
    models::user::{
        builder::user_builder::{UserBuilder, UserBuilderTrait},
        director::{UserDirector, UserDirectorTrait},
        model::user::{User, UserTrait},
    },
    utils::error::{internal_error, ErrorResponse},
};

use super::repository::{AuthenticationRepository, AuthenticationRepositoryTrait};

pub trait AuthenticationServiceTrait {
    fn new() -> Self;
    async fn login(
        &self,
        email: String,
        password: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)>;
    async fn register(
        &self,
        email: String,
        password: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)>;
    fn verify_password(&self, password: String, hash_password: String)
        -> Result<bool, BcryptError>;
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
    fn new() -> Self {
        Self {
            repository: AuthenticationRepository::default(),
        }
    }

    async fn login(
        &self,
        email: String,
        password: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let repository = self.repository();
        let user = repository.find_by_email(email, pool).await?;
        match self.verify_password(password, user.password().to_owned()) {
            Ok(success) => {
                if !success {
                    let status_code = StatusCode::BAD_REQUEST;
                    return Err((
                        status_code,
                        Json(
                            ErrorResponse::new()
                                .with_statuscode(status_code)
                                .with_message("Invalid credentials".to_string())
                                .build(),
                        ),
                    ));
                }
                Ok(user)
            }
            Err(e) => Err(internal_error(e)),
        }
    }

    async fn register(
        &self,
        email: String,
        password: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let repository = self.repository();
        let mut builder = UserBuilder::default();
        _ = UserDirector::construct_register_customer_user(&mut builder, email, password);
        let registered_user: User = builder.build();
        _ = repository.create_user(&registered_user, pool).await?;
        Ok(registered_user)
    }

    fn verify_password(
        &self,
        password: String,
        hash_password: String,
    ) -> Result<bool, BcryptError> {
        verify(password, hash_password.as_str())
    }
}
