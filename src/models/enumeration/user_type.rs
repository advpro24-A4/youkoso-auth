use std::fmt;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::PgValue,
    serialize::{self, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};

use crate::models::db::schema::sql_types;

#[derive(Debug, Clone, AsExpression, FromSqlRow, Serialize, Deserialize, PartialEq, Eq)]
#[diesel(sql_type = sql_types::UserRole)]
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

impl ToSql<sql_types::UserRole, diesel::pg::Pg> for UserRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> serialize::Result {
        let v = match self {
            UserRole::User => String::from("USER"),
            UserRole::Admin => String::from("ADMIN"),
            UserRole::Customer => String::from("CUSTOMER"),
        };
        <String as ToSql<Text, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

impl FromSql<sql_types::UserRole, diesel::pg::Pg> for UserRole {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        let role_str = std::str::from_utf8(value.as_bytes())?;
        match role_str {
            "USER" => Ok(UserRole::User),
            "ADMIN" => Ok(UserRole::Admin),
            "CUSTOMER" => Ok(UserRole::Customer),
            _ => Err(format!("Unrecognized variant {}", role_str).into()),
        }
    }
}
