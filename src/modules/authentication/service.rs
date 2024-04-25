use crate::models::user::model::user::User;

trait AuthenticationService {
    fn login(user: User) -> User;
    fn register(user: User) -> User;
    fn verify_password(password: String) -> bool;
}

pub struct AuthenticationServiceImpl {}

impl AuthenticationService for AuthenticationServiceImpl {
    fn login(user: User) -> User {
        unimplemented!()
    }

    fn register(user: User) -> User {
        unimplemented!()
    }

    fn verify_password(password: String) -> bool {
        unimplemented!()
    }
}
