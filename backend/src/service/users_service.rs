use crate::utils::token::Authorization;
use crate::Context;
use crate::{database::entities::user_entity::User, utils::token::create_token};

use bcrypt;
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::FromRow;

use crate::service::errors::server_error::ServerError;

pub async fn new_user(
    ctx: &Context,
    username: &String,
    password: &String,
    role: &String,
) -> Result<LoginResponse, ServerError> {
    let config = &ctx.config;
    let conn = &ctx.pool;
    let hashed_password = hash_password(password);

    if hashed_password.is_err() {
        return Err(ServerError::InternalError);
    }

    let new_user = sqlx::query_as::<_, (i32, String, String)>(
        "INSERT INTO user(username, password, role) VALUES(?, ?, ?) RETURNING id, username, role",
    )
    .bind(username)
    .bind(hashed_password.as_ref().unwrap())
    .bind(role)
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

    return Ok(LoginResponse {
        id,
        username,
        role,
        token: token.unwrap(),
    });
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct MainPageUser {
    id: i32,
    username: String,
    uses_password: i8,
}

pub async fn get_users(ctx: &Context) -> Result<Vec<MainPageUser>, ServerError> {
    let users = sqlx::query_as!(MainPageUser, "SELECT id, username, uses_password FROM user")
        .fetch_all(&ctx.pool)
        .await;

    return match users {
        Err(_) => Err(ServerError::InternalError),
        Ok(users) => Ok(users),
    };
}

#[derive(Serialize)]
pub struct UsesPasswordReponse {
    uses_password: i8,
}

pub async fn user_uses_password(
    ctx: &Context,
    user_id: i32,
) -> Result<UsesPasswordReponse, ServerError> {
    let res = sqlx::query_as!(
        UsesPasswordReponse,
        "SELECT uses_password FROM user WHERE id = ?",
        &user_id
    )
    .fetch_one(&ctx.pool)
    .await;

    return match res {
        Err(_) => Err(ServerError::NotFound),
        Ok(user) => Ok(UsesPasswordReponse {
            uses_password: user.uses_password,
        }),
    };
}

#[derive(Serialize, FromRow)]
pub struct LoginResponse {
    id: i32,
    username: String,
    role: String,
    token: String,
}

pub async fn login(
    ctx: &Context,
    user_id: i32,
    password: Option<&String>,
) -> Result<LoginResponse, ServerError> {
    let config = &ctx.config;
    let conn = &ctx.pool;

    let res = sqlx::query_as!(User, "SELECT * FROM user WHERE id=?", user_id)
        .fetch_one(conn)
        .await;

    if res.is_err() {
        return Err(ServerError::NotFound);
    }

    let user = res.unwrap();

    if user.uses_password == 1 {
        if password.is_none() {
            return Err(ServerError::ValidationError {
                field: String::from("password"),
            });
        }

        if verify(password.as_ref().unwrap(), user.password.as_str()).is_err() {
            return Err(ServerError::NotFound);
        };
    }

    let token = create_token(user.id, &config.jwt_secret);
    if token.is_err() {
        return Err(ServerError::NotFound);
    }

    return Ok(LoginResponse {
        id: user.id,
        username: user.username,
        role: user.role,
        token: token.unwrap(),
    });
}

#[derive(Serialize, FromRow)]
pub struct ViewableUser {
    id: i32,
    username: String,
    role: String,
}

pub async fn get_user(auth: &Authorization, ctx: &Context) -> Result<ViewableUser, ServerError> {
    let user = sqlx::query_as!(
        ViewableUser,
        "SELECT id, username, role FROM user WHERE id=?",
        auth.sub
    )
    .fetch_one(&ctx.pool)
    .await;

    return match user {
        Err(_) => Err(ServerError::NotFound),
        Ok(user) => Ok(user),
    };
}

// the cost was not really tested, so this is probably not good for a real app,
// but also this is only going to be run by on local networks.
pub fn hash_password(password: &String) -> Result<String, ServerError> {
    return bcrypt::hash(password, 8).map_err(|_error| ServerError::InternalError);
}

pub fn verify(password: &String, hash: &str) -> Result<bool, ServerError> {
    return bcrypt::verify(password, hash).map_err(|_error| ServerError::InternalError);
}
