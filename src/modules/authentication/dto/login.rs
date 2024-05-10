use serde::{Deserialize, Serialize};

use crate::models::user::model::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
    pub user: User,
    pub token: String,
}
