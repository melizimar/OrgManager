use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub name: String,
    pub password: String,
    #[serde(with = "date_format")]
    pub birth_date: Date,
    pub role: String,
}

impl UserDTO {
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
