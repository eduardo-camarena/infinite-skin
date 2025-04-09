use sea_orm::{DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::prelude::User")]
pub struct MainPageUser {
    pub id: i32,
    pub username: String,
    pub uses_password: i8,
    pub role: String,
}

#[derive(Serialize, Deserialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::prelude::User")]
pub struct UsesPasswordUser {
    pub uses_password: i8,
}

#[derive(Serialize, Deserialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::prelude::User")]
pub struct UserWithPassword {
    pub id: i32,
    pub username: String,
    pub uses_password: i8,
    pub role: String,
    pub password: String,
}
