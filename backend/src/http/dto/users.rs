use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDTO {
    pub id: i32,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct NewUserDTO {
    pub username: String,
    pub password: String,
    pub role: String,
}
