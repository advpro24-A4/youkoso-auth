use std::fmt;

pub enum UserRole {
    Admin,
    Customer,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "ADMIN"),
            UserRole::Customer => write!(f, "CUSTOMER"),
        }
    }
}
