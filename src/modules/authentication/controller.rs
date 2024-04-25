use axum::{routing::get, Router};

use crate::AppState;

pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new().route("/", get(auth)).with_state(state)
}

async fn auth() -> &'static str {
    "auth is running"
}
