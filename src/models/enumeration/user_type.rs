use std::fmt;

#[derive(Debug, Clone)]
pub enum UserRole {
    User,
    Admin,
    Customer,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::User => write!(f, "USER"),
            UserRole::Admin => write!(f, "ADMIN"),
            UserRole::Customer => write!(f, "CUSTOMER"),
        }
    }
}
