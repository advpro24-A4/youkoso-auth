use axum::{http::StatusCode, Json};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    models::user::model::token::TokenClaims,
    utils::{
        config::config,
        error::{unauthorized_error_expired, ErrorResponse},
    },
};

pub async fn validate_token(token: String) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    let config = config().await;
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret().as_ref()),
        &Validation::default(),
    )
    .map_err(unauthorized_error_expired)?
    .claims;
    Ok(claims.sub.to_owned())
}
