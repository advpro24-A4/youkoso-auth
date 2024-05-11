use axum::{http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;

use crate::{
    models::user::model::{profile::Profile, user::User},
    utils::error::ErrorResponse,
};

use super::repository::{ProfileRepository, ProfileRepositoryTrait};

#[derive(Default)]
pub struct ProfileSercvice {
    repository: ProfileRepository,
}

pub trait ProfileServiceTrait {
    fn new() -> Self;
    async fn create_profile(
        &self,
        user_id: String,
        profile: Profile,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)>;
}

trait PrivateProfileServiceTrait {
    fn repository(&self) -> &ProfileRepository;
}

impl PrivateProfileServiceTrait for ProfileSercvice {
    fn repository(&self) -> &ProfileRepository {
        &self.repository
    }
}

impl ProfileServiceTrait for ProfileSercvice {
    fn new() -> Self {
        Self {
            repository: ProfileRepository::default(),
        }
    }
    async fn create_profile(
        &self,
        user_id: String,
        profile: Profile,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let repository = self.repository();
        let user = repository.update_profile(user_id, profile, pool).await?;
        Ok(user)
    }
}
