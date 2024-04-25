use crate::models::enumeration::user_type::UserRole;
use crate::models::user::{User, UserTrait};

fn init_user() -> User {
    User::new(
        "hkalipaksi@kaka.com".to_owned(),
        "sukses".to_owned(),
        UserRole::Customer,
    )
}

#[test]
fn test_general_user_getter() {
    let user: User = init_user();
    let cloned_user: User = user.clone();

    _ = user.set_phone_number("0812131380037".to_owned());
    assert_eq!(user.get_id(), cloned_user.get_id());
    assert_eq!(user.get_email(), "hkalipaksi@kaka.com");
    assert_eq!(user.get_password(), "sukses");
}

#[test]
fn test_set_get_name() {
    let user: User = init_user();
    _ = user.set_name("Haekal".to_owned());
    assert_eq!(user.get_name(), "Haekal");
}

#[test]
fn test_set_get_address() {
    let user: User = init_user();
    _ = user.set_address("Jl. Jambu X No.19".to_owned());
    assert_eq!(user.get_address(), "Jl. Jambu X No.19")
}

#[test]
fn test_set_get_birth_date() {
    let user: User = init_user();
    _ = user.set_birth_date(chrono::NaiveDate::from_ymd_opt(2024, 12, 24).unwrap());
    assert_eq!(
        user.get_birth_date(),
        chrono::NaiveDate::from_ymd_opt(2024, 12, 24).unwrap()
    );
}

#[test]
fn test_set_get_username() {
    let user: User = init_user();
    _ = user.set_username("mhmdhaekal".to_owned());
    assert_eq!(user.get_phone_number(), "081213180037".to_owned());
}

#[test]
fn test_create_customer_role() {
    let customer_user = User::new(
        "hkalipaksi@kaka.com".to_owned(),
        "sukses".to_owned(),
        UserRole::Customer,
    );

    assert_eq!(
        customer_user.get_role().to_string(),
        UserRole::Customer.to_string()
    )
}

#[test]
fn test_create_admin_role() {
    let admin_user = User::new(
        "hkalipaksi@kaka.com".to_owned(),
        "sukses".to_owned(),
        UserRole::Admin,
    );

    assert_eq!(
        admin_user.get_role().to_string(),
        UserRole::Admin.to_string()
    )
}
