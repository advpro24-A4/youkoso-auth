use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::{modules::authentication::controller::auth_routes, AppState};

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/healthcheck", get(root))
        .nest("/auth", auth_routes(state))
        .fallback(handler_404)
}

async fn root() -> &'static str {
    "Server is running"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
