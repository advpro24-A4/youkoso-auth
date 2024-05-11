use axum::{
    body::Body,
    extract::{self, Request, State},
    http::{header, StatusCode},
    routing::{get, post},
    Json, Router,
};
use validator::Validate;

use crate::{
    models::user::model::user::UserTrait,
    modules::authentication::service::{AuthenticationService, AuthenticationServiceTrait},
};

use crate::{
    modules::authentication::dto::{
        login::{LoginDTO, LoginResponse},
        register::{RegisterDTO, RegisterResponse},
    },
    utils::error::ErrorResponse,
    AppState,
};

use super::dto::{logout::LogoutResponse, verify::VerifyResponse};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify", get(verify))
        .route("/logout", get(logout))
        .with_state(state)
}

async fn register(
    State(state): State<AppState>,
    extract::Json(request): extract::Json<RegisterDTO>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<ErrorResponse>)> {
    match request.validate() {
        Ok(_) => {
            let service = AuthenticationService::new();
            let user = service
                .register(request.email, request.password, &state.pool)
                .await?;
            let response: RegisterResponse = RegisterResponse {
                email: user.email().to_owned(),
                message: String::from("Success register"),
            };
            Ok(Json(response))
        }
        Err(e) => {
            let status_code = StatusCode::BAD_REQUEST;
            let error = Json(
                ErrorResponse::new()
                    .with_message(e.to_string())
                    .with_statuscode(status_code)
                    .build(),
            );
            Err((status_code, error))
        }
    }
}

async fn login(
    State(state): State<AppState>,
    extract::Json(request): extract::Json<LoginDTO>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    match request.validate() {
        Ok(_) => {
            let service = AuthenticationService::new();
            let user = service
                .login(request.email, request.password, &state.pool)
                .await?;
            let token = service.create_token(&user, &state.pool).await?;
            let response: LoginResponse = LoginResponse {
                message: String::from("Success login"),
                user,
                token,
            };
            Ok(Json(response))
        }
        Err(e) => {
            let status_code = StatusCode::BAD_REQUEST;
            let error = Json(
                ErrorResponse::new()
                    .with_message(e.to_string())
                    .with_statuscode(status_code)
                    .build(),
            );
            Err((status_code, error))
        }
    }
}

async fn verify(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<Json<VerifyResponse>, (StatusCode, Json<ErrorResponse>)> {
    let service = AuthenticationService::new();
    let header_token = req.headers().get(header::AUTHORIZATION);
    let user = service.auth(header_token, &state.pool).await?;
    Ok(Json(VerifyResponse {
        message: "Success get user".to_string(),
        user,
    }))
}

async fn logout(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Result<Json<LogoutResponse>, (StatusCode, Json<ErrorResponse>)> {
    let service = AuthenticationService::new();
    let header_token = req.headers().get(header::AUTHORIZATION);
    _ = service.logout(header_token, &state.pool).await?;
    Ok(Json(LogoutResponse {
        message: "success logout, see you later".to_string(),
    }))
}
