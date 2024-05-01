use crate::{
    models::db::{entity::user_schema::UserInsertSchema, schema::users::dsl::*},
    utils::error::internal_error,
};
use axum::http::StatusCode;
use deadpool_diesel::postgres::Pool;
use diesel::{associations::HasTable, insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::models::user::model::user::{User, UserTrait};

trait AuthenticationRepositoryTrait {
    async fn create_user(user: User, pool: &Pool) -> Result<User, (StatusCode, String)>;
    fn find_by_email(email: String) -> User;
    fn find_by_id(id: String) -> User;
}

pub struct AuthenticationRepository {}

impl AuthenticationRepositoryTrait for AuthenticationRepository {
    async fn create_user(user: User, pool: &Pool) -> Result<User, (StatusCode, String)> {
        let encrypted_password = user.encrypt_password();
        let returned_user = user.clone();
        let conn = pool.get().await.map_err(internal_error)?;
        let _ = conn
            .interact(move |conn| {
                insert_into(users)
                    .values((
                        id.eq(user.id()),
                        email.eq(user.email()),
                        password.eq(encrypted_password),
                        role.eq(user.role()),
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
