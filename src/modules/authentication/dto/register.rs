use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterDTO {
    pub email: String,
    pub password: String,
    pub confirmation_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub email: String,
    pub message: String,
}
