use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Admin {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}
