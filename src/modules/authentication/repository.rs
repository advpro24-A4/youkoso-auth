use crate::{
    models::{
        db::{entity::user_schema::UserSelectSchema, schema::users::dsl::*},
        enumeration::user_type::UserRole,
        user::{
            builder::user_builder::{UserBuilder, UserBuilderTrait},
            model::profile::{Profile, ProfileTrait},
        },
    },
    utils::error::{internal_error, ErrorResponse},
};
use axum::{http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::user::model::user::{User, UserTrait};

pub trait AuthenticationRepositoryTrait {
    async fn create_user(
        &self,
        user: &User,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)>;
    async fn find_by_email(
        &self,
        email: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)>;
    fn find_by_id(id: String) -> User;
}

#[derive(Debug, Clone, Default)]
pub struct AuthenticationRepository {}

impl AuthenticationRepositoryTrait for AuthenticationRepository {
    async fn create_user(
        &self,
        user: &User,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let encrypted_password = user.encrypt_password();
        let insert_user = user.clone();
        let returned_user = user.clone();
        dbg!("halo");
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

    async fn find_by_email(
        &self,
        user_email: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let conn = pool.get().await.map_err(internal_error)?;

        let res = conn
            .interact(|conn| {
                users
                    .filter(email.eq(user_email))
                    .select(UserSelectSchema::as_select())
                    .get_result(conn)
            })
            .await
            .map_err(internal_error)?;
        match res {
            Ok(user_select_schema) => {
                if user_select_schema.username.is_none() {
                    let login_user = UserBuilder::default()
                        .with_id(user_select_schema.id)
                        .with_email(user_select_schema.email)
                        .with_password(user_select_schema.password)
                        .with_role(user_select_schema.role.unwrap_or(UserRole::User))
                        .build();
                    return Ok(login_user);
                }
                let profile = Profile::new(
                    user_select_schema.name.unwrap(),
                    user_select_schema.username.unwrap(),
                    user_select_schema.address.unwrap(),
                    user_select_schema.birth_date.unwrap(),
                    user_select_schema.phone_number.unwrap(),
                );
                let user = UserBuilder::default()
                    .with_id(user_select_schema.id)
                    .with_email(user_select_schema.email)
                    .with_password(user_select_schema.password)
                    .with_role(user_select_schema.role.unwrap_or(UserRole::User))
                    .with_profile(profile)
                    .build();
                Ok(user)
            }
            Err(_) => {
                let status_code = StatusCode::NOT_FOUND;
                let error_response = ErrorResponse::new()
                    .with_statuscode(status_code)
                    .with_message("User not found".to_string())
                    .build();
                let json_response = Json(error_response);
                Err((status_code, json_response))
            }
        }
    }

    fn find_by_id(_user_id: String) -> User {
        unimplemented!();
    }
}
