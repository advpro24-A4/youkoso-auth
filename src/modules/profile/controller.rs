use axum::extract::State;
use axum::http::HeaderMap;
use axum::{
    extract::{self},
    http::{header, StatusCode},
    routing::post,
    Json, Router,
};
use chrono::NaiveDate;
use validator::Validate;

use crate::{
    models::user::model::{
        profile::{Profile, ProfileTrait},
        user::UserTrait,
    },
    modules::authentication::service::{AuthenticationService, AuthenticationServiceTrait},
    utils::error::ErrorResponse,
    AppState,
};

use super::{
    dto::create::{ProfileCreateDTO, ProfileCreateResponse},
    service::{ProfileSercvice, ProfileServiceTrait},
};

pub fn profile_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_profile))
        .with_state(state)
}

async fn create_profile(
    headers: HeaderMap,
    State(state): State<AppState>,
    extract::Json(request): extract::Json<ProfileCreateDTO>,
) -> Result<Json<ProfileCreateResponse>, (StatusCode, Json<ErrorResponse>)> {
    match request.validate() {
        Ok(_) => match NaiveDate::parse_from_str(&request.birth_date, "%Y-%m-%d") {
            Ok(birth_date) => {
                let service = ProfileSercvice::new();
                let auth_service = AuthenticationService::new();
                let header_token = headers.get(header::AUTHORIZATION);

                let user = auth_service.auth(header_token, &state.pool).await?;

                let profile = Profile::new(
                    request.name,
                    request.username,
                    request.address,
                    birth_date,
                    request.phone_number,
                );
                let user = service
                    .create_profile(user.id().to_owned(), profile, &state.pool)
                    .await?;

                Ok(Json(ProfileCreateResponse {
                    message: "Success modified profile".to_string(),
                    user,
                }))
            }
            Err(_) => {
                let error = Json(
                    ErrorResponse::new()
                        .with_statuscode(StatusCode::BAD_REQUEST)
                        .with_message("Invalid date format".to_string())
                        .build(),
                );
                return Err((StatusCode::BAD_REQUEST, error));
            }
        },
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
