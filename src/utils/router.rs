use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::{
    modules::{authentication::controller::auth_routes, profile::controller::profile_routes},
    AppState,
};

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/healthcheck", get(root))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/profile", profile_routes(state.clone()))
        .fallback(handler_404)
        .layer(TraceLayer::new_for_http())
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
