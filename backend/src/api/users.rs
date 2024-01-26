use crate::utils::token::get_authorization;
use crate::AppData;
use crate::{database::entities::user_entity::User, utils::token::create_token};

use actix_web::HttpRequest;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{Data, Json, Path},
    Responder,
};
use bcrypt;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::FromRow;

use crate::api::errors::server_error::ServerError;

#[derive(Deserialize)]
pub struct NewUserPayload {
    username: String,
    password: String,
    role: String,
}

#[post("/new")]
pub async fn new_user(app_data: Data<AppData>, payload: Json<NewUserPayload>) -> impl Responder {
    let config = &app_data.config;
    let conn = &app_data.pool;
    let hashed_password = hash_password(&payload.password);

    if hashed_password.is_err() {
        return Err(ServerError::InternalError);
    }

    println!(
        "{} {} {}",
        payload.username,
        hashed_password.as_ref().unwrap(),
        payload.role
    );

    let new_user = sqlx::query_as::<_, (i32, String, String)>(
        "INSERT INTO user(username, password, role) VALUES(?, ?, ?) RETURNING id, username, role",
    )
    .bind(&payload.username)
    .bind(hashed_password.as_ref().unwrap())
    .bind(&payload.role)
    .fetch_one(conn)
    .await;

    if new_user.is_err() {
        return Err(ServerError::InternalError);
    }

    let (id, username, role) = new_user.unwrap();
    let token = create_token(id, &config.jwt_secret);

    if token.is_err() {
        return Err(ServerError::InternalError);
    }

    return Ok(Json(LoginResponse {
        id,
        username,
        role,
        token: token.unwrap(),
    }));
}

#[derive(Serialize, Deserialize, FromRow)]
struct MainPageUser {
    id: i32,
    username: String,
    uses_password: i8,
}

#[get("")]
pub async fn get_users(app_data: Data<AppData>) -> impl Responder {
    let conn = &app_data.pool;

    let users = sqlx::query_as!(MainPageUser, "SELECT id, username, uses_password FROM user")
        .fetch_all(conn)
        .await;

    return match users {
        Err(_) => Err(ServerError::InternalError),
        Ok(users) => Ok(Json(users)),
    };
}

#[derive(Serialize)]
struct UsesPasswordReponse {
    uses_password: i8,
}

#[get("/{user_id}/uses-password")]
pub async fn user_uses_password(app_data: Data<AppData>, path: Path<i32>) -> impl Responder {
    let conn = &app_data.pool;
    let user_id = path.into_inner();

    let res = sqlx::query_as!(
        UsesPasswordReponse,
        "SELECT uses_password FROM user WHERE id = ?",
        &user_id
    )
    .fetch_one(conn)
    .await;

    return match res {
        Err(_) => Err(ServerError::NotFound),
        Ok(user) => Ok(Json(UsesPasswordReponse {
            uses_password: user.uses_password,
        })),
    };
}

#[derive(Deserialize)]
pub struct LoginPayload {
    id: i32,
    pub password: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct LoginResponse {
    id: i32,
    username: String,
    role: String,
    token: String,
}

#[post("/login")]
pub async fn login(app_data: Data<AppData>, payload: Json<LoginPayload>) -> impl Responder {
    let config = &app_data.config;
    let conn = &app_data.pool;

    let res = sqlx::query_as!(User, "SELECT * FROM user WHERE id=?", payload.id)
        .fetch_one(conn)
        .await;

    if res.is_err() {
        return Err(ServerError::NotFound);
    }

    let user = res.unwrap();

    if user.uses_password == 1 {
        if payload.password.is_none() {
            return Err(ServerError::ValidationError {
                field: String::from("password"),
            });
        }

        if verify(payload.password.as_ref().unwrap(), user.password.as_str()).is_err() {
            return Err(ServerError::NotFound);
        };
    }

    let token = create_token(user.id, &config.jwt_secret);
    if token.is_err() {
        return Err(ServerError::NotFound);
    }

    return Ok(Json(LoginResponse {
        id: user.id,
        username: user.username,
        role: user.role,
        token: token.unwrap(),
    }));
}

#[derive(Serialize, FromRow)]
pub struct ViewableUser {
    id: i32,
    username: String,
    role: String,
}

#[get("/me")]
pub async fn get_user(app_data: Data<AppData>, req: HttpRequest) -> impl Responder {
    let conn = &app_data.pool;
    let auth = get_authorization(&req).unwrap();

    println!("{}", auth.sub);

    let user = sqlx::query_as!(
        ViewableUser,
        "SELECT id, username, role FROM user WHERE id=?",
        auth.sub
    )
    .fetch_one(conn)
    .await;

    return match user {
        Err(_) => Err(ServerError::NotFound),
        Ok(user) => Ok(Json(user)),
    };
}

// the cost was not really tested, so this is probably not good for a real app,
// but also this is only going to be run by on local networks.
pub fn hash_password(password: &String) -> Result<String, StatusCode> {
    return bcrypt::hash(password, 8).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR);
}

pub fn verify(password: &String, hash: &str) -> Result<bool, StatusCode> {
    return bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR);
}
