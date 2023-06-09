use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}
