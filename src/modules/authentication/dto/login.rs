use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::user::model::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(
        length(min = 1, message = "Can not be empty"),
        email(message = "please input valid email")
    )]
    pub email: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
    pub user: User,
    pub token: String,
}
