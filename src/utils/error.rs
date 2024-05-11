use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

pub fn internal_error<E>(err: E) -> (StatusCode, Json<ErrorResponse>)
where
    E: std::error::Error,
{
    let status_code = StatusCode::INTERNAL_SERVER_ERROR;
    let error_response = ErrorResponse::new()
        .with_statuscode(status_code)
        .with_message(err.to_string())
        .build();
    let json_response = Json(error_response);
    (status_code, json_response)
}

pub fn unauthorized_error() -> (StatusCode, Json<ErrorResponse>) {
    let status_code = StatusCode::UNAUTHORIZED;
    let error_response = Json(
        ErrorResponse::new()
            .with_statuscode(status_code)
            .with_message("Invalid token".to_string())
            .build(),
    );
    (status_code, error_response)
}

pub fn unauthorized_error_expired<E>(err: E) -> (StatusCode, Json<ErrorResponse>)
where
    E: std::error::Error,
{
    let status_code = StatusCode::UNAUTHORIZED;
    let error_response = Json(
        ErrorResponse::new()
            .with_statuscode(status_code)
            .with_message(err.to_string())
            .build(),
    );
    (status_code, error_response)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    message: Option<String>,
    status_code: Option<String>,
}

impl ErrorResponse {
    pub fn new() -> Self {
        Self {
            message: None,
            status_code: None,
        }
    }

    pub fn with_statuscode(&mut self, status: StatusCode) -> &mut Self {
        self.status_code = Some(status.to_string());
        self
    }

    pub fn with_message(&mut self, message: String) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn build(&mut self) -> Self {
        Self {
            message: self.message.to_owned(),
            status_code: self.status_code.to_owned(),
        }
    }
}
