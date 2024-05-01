use crate::{models::db::schema::users::dsl::*, utils::error::internal_error};
use axum::http::StatusCode;
use deadpool_diesel::postgres::Pool;
use diesel::{insert_into, ExpressionMethods, RunQueryDsl};

use crate::models::user::model::user::{User, UserTrait};

pub trait AuthenticationRepositoryTrait {
    async fn create_user(&self, user: &User, pool: &Pool) -> Result<User, (StatusCode, String)>;
    fn find_by_email(email: String) -> User;
    fn find_by_id(id: String) -> User;
}

#[derive(Debug, Clone, Default)]
pub struct AuthenticationRepository {}

impl AuthenticationRepositoryTrait for AuthenticationRepository {
    async fn create_user(&self, user: &User, pool: &Pool) -> Result<User, (StatusCode, String)> {
        let encrypted_password = user.encrypt_password();
        let insert_user = user.clone();
        let returned_user = user.clone();
        let conn = pool.get().await.map_err(internal_error)?;
        let _ = conn
            .interact(move |conn| {
                insert_into(users)
                    .values((
                        id.eq(insert_user.id()),
                        email.eq(insert_user.email()),
                        password.eq(encrypted_password),
                        role.eq(insert_user.role()),
                    ))
                    .execute(conn)
            })
            .await;
        Ok(returned_user)
    }

    fn find_by_email(user_email: String) -> User {
        unimplemented!();
    }

    fn find_by_id(user_id: String) -> User {
        unimplemented!();
    }
}
