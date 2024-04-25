use crate::models::{
    enumeration::user_type::UserRole,
    user::{
        builder::user_builder::{UserBuilder, UserBuilderTrait},
        director::{UserDirector, UserDirectorTrait},
        model::{
            profile::{Profile, ProfileTrait},
            user::{User, UserTrait},
        },
    },
};

#[test]
fn test_create_register_customer() {
    let mut builder = UserBuilder::default();
    _ = UserDirector::construct_register_customer_user(
        &mut builder,
        "hkalipaksi@outlook.com".to_owned(),
        "Xpaksi19".to_owned(),
    );
    let user_customer: User = builder.build();

    assert_eq!(
        user_customer.role().to_string(),
        UserRole::Customer.to_string()
    );
    assert_eq!(user_customer.profile().is_none(), true);
}

#[test]
fn test_create_admin() {
    let mut builder = UserBuilder::default();
    _ = UserDirector::construct_admin_user(
        &mut builder,
        "hkalipaksi@outlook.com".to_owned(),
        "Xpaksi19".to_owned(),
    );
    let user_admin: User = builder.build();

    assert_eq!(user_admin.role().to_string(), UserRole::Admin.to_string());
    assert_eq!(user_admin.profile().is_none(), true);
}

#[test]
fn test_create_profile_customer() {
    let mut builder = UserBuilder::default();
    _ = UserDirector::construct_register_customer_user(
        &mut builder,
        "hkalipaksi@outlook.com".to_owned(),
        "Xpaksi19".to_owned(),
    );
    let registered_customer: User = builder.build();

    let profile = Profile::new(
        "Haekal".to_string(),
        "hkalipaksi".to_string(),
        "Jl. Jambu X No.19".to_string(),
        chrono::NaiveDate::from_ymd_opt(2004, 04, 24).unwrap(),
        "081213180037".to_string(),
    );

    let mut builder = UserBuilder::default();
    _ = UserDirector::construct_customer_user(&mut builder, registered_customer, profile);
    let customer_user: User = builder.build();

    assert_eq!(
        customer_user.role().to_string(),
        UserRole::Customer.to_string()
    );

    assert_eq!(customer_user.profile().is_none(), false);

    let user_profile = customer_user.profile().as_ref().unwrap();

    assert_eq!(user_profile.name(), "Haekal");
    assert_eq!(user_profile.username(), "hkalipaksi");
    assert_eq!(user_profile.address(), "Jl. Jambu X No.19");
    assert_eq!(
        user_profile.birth_date().to_string(),
        chrono::NaiveDate::from_ymd_opt(2004, 04, 24)
            .unwrap()
            .to_string()
    );
    assert_eq!(user_profile.phone_number(), "081213180037")
}
