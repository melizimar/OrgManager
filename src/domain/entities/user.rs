use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    #[serde(with = "date_format")]
    pub birth_date: Date,
    pub role: String,
}

impl User {
    pub fn new<N, P, R>(name: N, password: P, birth_date: Date, role: R) -> Self
    where
        N: Into<String>,
        P: Into<String>,
        R: Into<String>,
    {
        Self {
            id: Uuid::now_v7(),
            name: name.into(),
            password: password.into(),
            birth_date,
            role: role.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_user_creation_success() {
        let user = User::new(
            "User1",
            "Password1",
            time::macros::date!(1990 - 01 - 01),
            "Admin",
        );

        assert_eq!(
            user,
            User {
                id: user.id,
                name: "User1".to_string(),
                password: "Password1".to_string(),
                birth_date: time::macros::date!(1990 - 01 - 01),
                role: "Admin".to_string(),
            }
        );
    }
}
