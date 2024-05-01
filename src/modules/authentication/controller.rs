use axum::{
    extract::{Form, State},
    routing::post,
    Json, Router,
};

use crate::{models::user::model::user::UserTrait, AppState};

use super::{
    dto::register::{RegisterDTO, RegisterResponse},
    service::{AuthenticationService, AuthenticationServiceTrait},
};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .with_state(state)
}

async fn register(
    State(state): State<AppState>,
    Form(request): Form<RegisterDTO>,
) -> Json<RegisterResponse> {
    let service = AuthenticationService::default();
    let user = service
        .register(request.email, request.password, &state.pool)
        .await
        .unwrap();
    let response: RegisterResponse = RegisterResponse {
        email: user.email().to_owned(),
        message: String::from("Success register"),
    };
    Json(response)
}
