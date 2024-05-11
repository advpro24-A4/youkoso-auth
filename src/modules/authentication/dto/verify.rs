use serde::{Deserialize, Serialize};

use crate::models::user::model::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub message: String,
    pub user: User,
}
