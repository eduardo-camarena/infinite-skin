use crate::database::entities::user_entity::User;
use crate::AppData;

use actix_web::{
    get,
    web::{Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};
use sqlx;

use crate::api::errors::{server_error::ServerError, user_error::UserError};

#[derive(Serialize, Deserialize)]
pub struct UserExists {
    it_works: bool,
}

#[derive(Serialize, Deserialize)]
struct MainPageUser {
    id: i32,
    username: String,
}

#[get("/users")]
pub async fn get_users(app_data: Data<AppData>) -> impl Responder {
    let conn = &app_data.pool;

    let users = sqlx::query_as!(MainPageUser, "SELECT id, username FROM user")
        .fetch_all(conn)
        .await;

    return match users {
        Err(_) => Err(ServerError::InternalError),
        Ok(users) => Ok(Json(users)),
    };
}

#[get("/me")]
pub async fn get_user(app_data: Data<AppData>) -> impl Responder {
    let conn = &app_data.pool;

    let user = sqlx::query_as!(User, "SELECT id, username, role FROM user WHERE id=?", 1)
        .fetch_one(conn)
        .await;

    return match user {
        Err(_) => Err(UserError::NotFound),
        Ok(user) => Ok(Json(user)),
    };
}
