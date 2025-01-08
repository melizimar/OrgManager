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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDTO {
    pub name: String,
    pub password: String,
    #[serde(with = "date_format")]
    pub birth_date: Date,
    pub role: String,
}

impl UserDTO {
    #[warn(dead_code)]
    pub fn new<N, P, R>(name: N, password: P, birth_date: Date, role: R) -> Self
    where
        N: Into<String>,
        P: Into<String>,
        R: Into<String>,
    {
        Self {
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
            "Matheus",
            "@aksjdcm@123",
            time::macros::date!(1999 - 10 - 12),
            "Administrator",
        );

        assert_eq!(
            user,
            User {
                id: user.id.clone(),
                name: "Matheus".to_string(),
                password: "@aksjdcm@123".to_string(),
                birth_date: time::macros::date!(1999 - 10 - 12),
                role: "Administrator".to_string(),
            }
        );
    }

    #[test]
    fn test_userdto_creation_success() {
        let user_dto = UserDTO::new(
            "Matheus",
            "@aksjdcm@123",
            time::macros::date!(1999 - 10 - 12),
            "Administrator",
        );

        assert_eq!(
            user_dto,
            UserDTO {
                name: "Matheus".to_string(),
                password: "@aksjdcm@123".to_string(),
                birth_date: time::macros::date!(1999 - 10 - 12),
                role: "Administrator".to_string(),
            }
        );

    }
}