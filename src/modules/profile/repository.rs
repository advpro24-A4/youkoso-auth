use axum::{http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::{update, ExpressionMethods, RunQueryDsl};

use crate::{
    models::{
        db::{entity::user_schema::UserSelectSchema, schema::users},
        user::{
            builder::user_builder::{UserBuilder, UserBuilderTrait},
            director::{UserDirector, UserDirectorTrait},
            model::{
                profile::{Profile, ProfileTrait},
                user::User,
            },
        },
    },
    utils::error::{internal_error, ErrorResponse},
};

#[derive(Default)]
pub struct ProfileRepository {}

pub trait ProfileRepositoryTrait {
    async fn update_profile(
        &self,
        user_id: String,
        profile: Profile,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)>;
}

impl ProfileRepositoryTrait for ProfileRepository {
    async fn update_profile(
        &self,
        user_id: String,
        profile: Profile,
        pool: &Pool,
    ) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
        let conn = pool.get().await.map_err(internal_error)?;
        let res = conn
            .interact(move |conn| {
                update(users::table)
                    .filter(users::dsl::id.eq(user_id))
                    .set((
                        users::dsl::name.eq(profile.name().clone()),
                        users::dsl::username.eq(profile.username().clone()),
                        users::dsl::address.eq(profile.address().clone()),
                        users::dsl::birth_date.eq(profile.birth_date().clone()),
                        users::dsl::phone_number.eq(profile.phone_number().clone()),
                    ))
                    .get_result::<UserSelectSchema>(conn)
            })
            .await
            .map_err(internal_error)?;

        match res {
            Ok(user) => {
                let profile = Profile::new(
                    user.name.unwrap(),
                    user.username.unwrap(),
                    user.address.unwrap(),
                    user.birth_date.unwrap(),
                    user.phone_number.unwrap(),
                );

                let mut builder = UserBuilder::default();
                _ = UserDirector::construct_register_customer_user_id(
                    &mut builder,
                    user.id,
                    user.email,
                    user.password,
                );
                let user = builder.build();
                _ = UserDirector::construct_customer_user(&mut builder, user, profile);
                let user = builder.build();
                Ok(user)
            }
            Err(e) => {
                match e {
                    diesel::result::Error::DatabaseError(kind, _) => match kind {
                        diesel::result::DatabaseErrorKind::UniqueViolation => {
                            let error = Json(
                                ErrorResponse::new()
                                    .with_statuscode(StatusCode::BAD_REQUEST)
                                    .with_message(
                                        "Username already used, please try another username"
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
}
