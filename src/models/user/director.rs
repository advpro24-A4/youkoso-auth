use crate::models::enumeration::user_type::UserRole;
use crate::models::user::model::user::User;

use crate::models::user::builder::user_builder::UserBuilderTrait;
use crate::models::user::model::profile::Profile;

pub struct UserDirector;

pub trait UserDirectorTrait {
    fn construct_admin_user(builder: &mut impl UserBuilderTrait, email: String, password: String);
    fn construct_register_customer_user(
        builder: &mut impl UserBuilderTrait,
        email: String,
        password: String,
    );

    fn construct_customer_user(builder: &mut impl UserBuilderTrait, user: User, profile: Profile);

    fn construct_register_customer_user_id(
        builder: &mut impl UserBuilderTrait,
        id: String,
        email: String,
        password: String,
    );
}

impl UserDirectorTrait for UserDirector {
    fn construct_admin_user(builder: &mut impl UserBuilderTrait, email: String, password: String) {
        builder
            .with_email(email)
            .with_password(password)
            .with_role(UserRole::Admin);
    }
    fn construct_register_customer_user(
        builder: &mut impl UserBuilderTrait,
        email: String,
        password: String,
    ) {
        builder
            .with_email(email)
            .with_password(password)
            .with_role(UserRole::Customer);
    }

    fn construct_register_customer_user_id(
        builder: &mut impl UserBuilderTrait,
        id: String,
        email: String,
        password: String,
    ) {
        builder
            .with_id(id)
            .with_email(email)
            .with_password(password)
            .with_role(UserRole::Customer);
    }

    fn construct_customer_user(builder: &mut impl UserBuilderTrait, user: User, profile: Profile) {
        builder
            .with_user(user)
            .with_role(UserRole::Customer)
            .with_profile(profile);
    }
}
