use crate::{
    models::{
        db::{
            entity::{token_schema::Token, user_schema::UserSelectSchema},
            schema::{tokens, users},
        },
        user::{
            builder::user_builder::{UserBuilder, UserBuilderTrait},
            director::{UserDirector, UserDirectorTrait},
            model::profile::{Profile, ProfileTrait},
        },
    },
    utils::error::{internal_error, ErrorResponse},
};
use axum::{http::StatusCode, Json};
use chrono::NaiveDateTime;
use deadpool_diesel::postgres::Pool;
use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use nanoid::nanoid;

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
    async fn find_by_id(
        &self,
        user_id: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)>;
    async fn create_token(
        &self,
        user: &User,
        created_token: &String,
        iat: NaiveDateTime,
        exp: NaiveDateTime,
        pool: &Pool,
    ) -> Result<Token, (StatusCode, Json<ErrorResponse>)>;
    async fn revoke_token(
        &self,
        token: String,
        pool: &Pool,
    ) -> Result<(), (StatusCode, Json<ErrorResponse>)>;
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
        let conn = pool.get().await.map_err(internal_error)?;
        let res = conn
            .interact(move |conn| {
                insert_into(users::table)
                    .values((
                        users::dsl::id.eq(insert_user.id()),
                        users::dsl::email.eq(insert_user.email()),
                        users::dsl::password.eq(encrypted_password),
                        users::dsl::role.eq(insert_user.role()),
                    ))
                    .get_result::<UserSelectSchema>(conn)
            })
            .await
            .map_err(internal_error)?;
        match res {
            Ok(user) => {
                let mut builder = UserBuilder::default();
                _ = UserDirector::construct_register_customer_user(
                    &mut builder,
                    user.email,
                    user.password,
                );
                let returned_user = builder.build();
                Ok(returned_user)
            }
            Err(e) => {
                match e {
                    diesel::result::Error::DatabaseError(kind, _) => match kind {
                        diesel::result::DatabaseErrorKind::UniqueViolation => {
                            let error = Json(
                                ErrorResponse::new()
                                    .with_statuscode(StatusCode::BAD_REQUEST)
                                    .with_message(
                                        "Email already registered, please try another email"
                                            .to_string(),
                                    )
                                    .build(),
                            );
                            return Err((StatusCode::BAD_REQUEST, error));
                        }
                        _ => {}
                    },
                    _ => {}
                }
                let error = Json(
                    ErrorResponse::new()
                        .with_statuscode(StatusCode::BAD_REQUEST)
                        .with_message("Please try again".to_string())
                        .build(),
                );
                Err((StatusCode::BAD_REQUEST, error))
            }
        }
    }

    async fn find_by_email(
        &self,
        user_email: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let conn = pool.get().await.map_err(internal_error)?;

        let res = conn
            .interact(|conn| {
                users::table
                    .filter(users::dsl::email.eq(user_email))
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
                        .with_role(user_select_schema.role)
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
                    .with_role(user_select_schema.role)
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

    async fn find_by_id(
        &self,
        user_id: String,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let conn = pool.get().await.map_err(internal_error)?;
        let res = conn
            .interact(|conn| {
                users::table
                    .filter(users::dsl::id.eq(user_id))
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
                        .with_role(user_select_schema.role)
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
                    .with_role(user_select_schema.role)
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

    async fn create_token(
        &self,
        user: &User,
        created_token: &String,
        iat: NaiveDateTime,
        exp: NaiveDateTime,
        pool: &Pool,
    ) -> Result<Token, (StatusCode, Json<ErrorResponse>)> {
        let insert_token = Token {
            id: nanoid!(),
            user_id: user.id().to_owned(),
            token: created_token.to_owned(),
            created_at: iat,
            expired_at: exp,
            status: "ACTIVE".to_string(),
        };
        let conn = pool.get().await.map_err(internal_error)?;
        let res = conn
            .interact(move |conn| {
                insert_into(tokens::table)
                    .values((
                        tokens::dsl::id.eq(insert_token.id),
                        tokens::dsl::token.eq(insert_token.token),
                        tokens::dsl::user_id.eq(insert_token.user_id),
                        tokens::dsl::created_at.eq(insert_token.created_at),
                        tokens::dsl::expired_at.eq(insert_token.expired_at),
                        tokens::dsl::status.eq(insert_token.status),
                    ))
                    .get_result::<Token>(conn)
            })
            .await
            .map_err(internal_error)?;

        match res {
            Ok(token) => Ok(token),
            Err(e) => {
                let status_code = StatusCode::BAD_REQUEST;
                let error_response = ErrorResponse::new()
                    .with_message(e.to_string())
                    .with_statuscode(status_code)
                    .build();
                let json_response = Json(error_response);
                Err((status_code, json_response))
            }
        }
    }

    async fn revoke_token(
        &self,
        token: String,
        pool: &Pool,
    ) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
        let conn = pool.get().await.map_err(internal_error)?;
        let res = conn
            .interact(move |conn| {
                update(tokens::table)
                    .filter(tokens::dsl::token.eq(token))
                    .set(tokens::status.eq("REVOKED".to_string()))
                    .get_result::<Token>(conn)
            })
            .await
            .map_err(internal_error)?;
        match res {
            Ok(_) => Ok(()),
            Err(err) => return Err(internal_error(err)),
        }
    }
}
