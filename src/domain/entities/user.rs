use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub role: String,
}

impl User {
    pub fn new<N, P, R>(name: N, password: P, role: R) -> Self
    where
        N: Into<String>,
        P: Into<String>,
        R: Into<String>,
    {
        Self {
            id: Uuid::now_v7(),
            name: name.into(),
            password: password.into(),
            role: role.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_user_creation_success() {
        let user = User::new("User1", "Password1", "Admin");

        assert_eq!(
            user,
            User {
                id: user.id,
                name: "User1".to_string(),
                password: "Password1".to_string(),
                role: "Admin".to_string(),
            }
        );
    }
}
