use axum::{
    body::Body,
    extract::{Form, Request, State},
    http::{header, StatusCode},
    routing::{get, post},
    Json, Router,
};

use crate::{models::user::model::user::UserTrait, utils::error::ErrorResponse, AppState};

use super::{
    dto::{
        login::{LoginDTO, LoginResponse},
        register::{RegisterDTO, RegisterResponse},
    },
    service::{AuthenticationService, AuthenticationServiceTrait},
};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify", get(verify))
        .with_state(state)
}

async fn register(
    State(state): State<AppState>,
    Form(request): Form<RegisterDTO>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<ErrorResponse>)> {
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

async fn login(
    State(state): State<AppState>,
    Form(request): Form<LoginDTO>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let service = AuthenticationService::new();
    let user = service
        .login(request.email, request.password, &state.pool)
        .await?;
    let response: LoginResponse = LoginResponse {
        message: String::from("Success login"),
        user,
        token: "Halo".to_string(),
    };
    Ok(Json(response))
}

async fn verify(
    State(_state): State<AppState>,
    req: Request<Body>,
) -> Result<Json<String>, (StatusCode, Json<ErrorResponse>)> {
    let token = req.headers().get(header::AUTHORIZATION);
    Ok(Json("Aman aja king".to_string()))
}
