use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::user::model::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProfileCreateDTO {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub address: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub birth_date: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileCreateResponse {
    pub message: String,
    pub user: User,
}
