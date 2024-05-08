use diesel::{debug_query, insert_into, pg::Pg, ExpressionMethods};

use crate::models::{
    db::schema::users::dsl::*,
    user::{
        builder::user_builder::{UserBuilder, UserBuilderTrait},
        director::{UserDirector, UserDirectorTrait},
        model::user::{User, UserTrait},
    },
};

#[test]
fn insert_registed_user() {
    let mut builder = UserBuilder::default();
    _ = UserDirector::construct_register_customer_user(
        &mut builder,
        "hkalipaksi@outlook.com".to_owned(),
        "Xpaksi19".to_owned(),
    );

    let registered_user: User = builder.build();

    let role_string = registered_user.role().clone().to_string().to_lowercase();

    let role_query = role_string
        .chars()
        .next()
        .unwrap()
        .to_uppercase()
        .chain(role_string.chars().skip(1))
        .collect::<String>();

    let encrypt_password = registered_user.encrypt_password();
    let query = insert_into(users).values((
        id.eq(registered_user.id()),
        email.eq(registered_user.email()),
        password.eq(encrypt_password.clone()),
        role.eq(registered_user.role()),
    ));

    let sql = format!("INSERT INTO \"users\" (\"id\", \"email\", \"password\", \"role\") VALUES ($1, $2, $3, $4) -- binds: [\"{}\", \"{}\", \"{}\", {}]", registered_user.id(), registered_user.email(), encrypt_password, role_query).to_string();
    let debug_query = debug_query::<Pg, _>(&query).to_string();
    assert_eq!(sql, debug_query);
}
