use crate::database::entities::user_entity::User;
use crate::AppData;

use actix_web::{
    get,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserExists {
    it_works: bool,
}

#[get("/")]
pub async fn get_user(app_data: Data<AppData>) -> Json<User> {
    let connection = &app_data.pool;

    let found_user = User {
        id: 1,
        username: String::from("lalo"),
        email: String::from("lalo@mail.com"),
        password: String::from("hello"),
    };

    Json(found_user)
}
