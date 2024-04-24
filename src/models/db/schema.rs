// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        #[max_length = 255]
        id -> Varchar,
        role -> Nullable<UserRole>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        birth_date -> Nullable<Date>,
        #[max_length = 255]
        phone_number -> Nullable<Varchar>,
    }
}
