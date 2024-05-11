use std::usize;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    http::{HeaderValue, StatusCode},
    Json,
};
use deadpool_diesel::postgres::Pool;
use jsonwebtoken::Header;
use jsonwebtoken::{encode, EncodingKey};

use crate::{
    models::user::{
        builder::user_builder::{UserBuilder, UserBuilderTrait},
        director::{UserDirector, UserDirectorTrait},
        model::{
            token::TokenClaims,
            user::{User, UserTrait},
        },
    },
    modules::common::jwt_utils::validate_token,
    utils::{
        config::config,
        error::{internal_error, unauthorized_error, ErrorResponse},
    },
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
    fn verify_password(&self, password: String, hash_password: String) -> bool;

    async fn auth(
        &self,
        token_header: Option<&HeaderValue>,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)>;
    async fn create_token(
        &self,
        user: &User,
        pool: &Pool,
    ) -> Result<String, (StatusCode, Json<ErrorResponse>)>;
    async fn logout(
        &self,
        token_header: Option<&HeaderValue>,
        pool: &Pool,
    ) -> Result<(), (StatusCode, Json<ErrorResponse>)>;
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
        let success = self.verify_password(password, user.password().to_owned());
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

    fn verify_password(&self, password: String, hash_password: String) -> bool {
        let parsed_hash = PasswordHash::new(hash_password.as_ref()).unwrap();
        let argon2 = Argon2::default();
        let verify = argon2.verify_password(password.as_ref(), &parsed_hash);
        match verify {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn auth(
        &self,
        token_header: Option<&HeaderValue>,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let token = match token_header {
            Some(header) => header.to_str().map_err(|_| unauthorized_error())?,
            None => return Err(unauthorized_error()),
        };
        if !token.starts_with("Bearer") {
            return Err(unauthorized_error());
        }
        let token = token[7..].trim().to_owned();
        let user_id = validate_token(token).await?;
        let repository = self.repository();
        let user = repository.find_by_id(user_id, pool).await?;
        Ok(user)
    }

    async fn create_token(
        &self,
        user: &User,
        pool: &Pool,
    ) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
        let now = chrono::Utc::now().naive_utc();
        let iat = now.and_utc().timestamp() as usize;
        let expired = now + chrono::Duration::minutes(60);
        let exp = expired.and_utc().timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: user.id().to_owned(),
            iat,
            exp,
        };
        let config = config().await;
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret().as_ref()),
        )
        .map_err(internal_error)?;
        let repository = self.repository();
        let return_token = repository
            .create_token(user, &token, now, expired, pool)
            .await?;
        Ok(return_token.token)
    }

    async fn logout(
        &self,
        token_header: Option<&HeaderValue>,
        pool: &Pool,
    ) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
        let token = match token_header {
            Some(header) => header.to_str().map_err(|_| unauthorized_error())?,
            None => return Err(unauthorized_error()),
        };
        if !token.starts_with("Bearer") {
            return Err(unauthorized_error());
        }
        let token = token[7..].trim().to_owned();
        let repository = self.repository();
        _ = repository.revoke_token(token, pool);
        Ok(())
    }
}
