use crate::models::user::model::user::User;

trait AuthenticationRepositoryTrait {
    fn create_user(user: User) -> User;
    fn find_by_email(email: String) -> User;
    fn find_by_id(id: String) -> User;
}

pub struct AuthenticationRepository {}

impl AuthenticationRepositoryTrait for AuthenticationRepository {
    fn find_by_id(id: String) -> User {
        unimplemented!();
    }

    fn create_user(user: User) -> User {
        unimplemented!();
    }

    fn find_by_email(email: String) -> User {
        unimplemented!();
    }
}
