use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterDTO {
    #[validate(
        length(min = 1, message = "Can not be empty"),
        email(message = "please input valid email")
    )]
    pub email: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
    #[validate(
        length(min = 1, message = "Can not be empty"),
        must_match(other = "password")
    )]
    pub confirmation_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub email: String,
    pub message: String,
}
