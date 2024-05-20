use crate::Context;
use crate::{
    database::models::user_model::{MainPageUser, UsesPasswordUser},
    utils::token::{create_token, Authorization},
};

use bcrypt;
use entity::prelude::User;
use sea_orm::{EntityTrait, Set};
use serde::Serialize;

use crate::service::errors::server_error::ServerError;

pub async fn new_user(
    ctx: &Context,
    username: &String,
    password: &String,
    role: &String,
) -> Result<LoginResponse, ServerError> {
    let config = &ctx.config;
    let hashed_password = hash_password(password);

    if hashed_password.is_err() {
        return Err(ServerError::InternalError);
    }

    let res = User::insert(entity::user::ActiveModel {
        username: Set(String::from(username)),
        password: Set(String::from(password)),
        role: Set(String::from(role)),
        ..Default::default()
    })
    .exec(&ctx.db)
    .await
    .map_err(|_| ServerError::InternalError)?;

    let new_user = User::find_by_id(res.last_insert_id)
        .one(&ctx.db)
        .await
        .map_err(|_| ServerError::InternalError)?
        .unwrap();

    let token = create_token(new_user.id, &config.jwt_secret);

    if token.is_err() {
        return Err(ServerError::InternalError);
    }

    return Ok(LoginResponse {
        id: new_user.id,
        username: new_user.username,
        role: new_user.role,
        token: token.unwrap(),
    });
}

pub async fn get_users(ctx: &Context) -> Result<Vec<MainPageUser>, ServerError> {
    let users = User::find()
        .into_partial_model::<MainPageUser>()
        .all(&ctx.db)
        .await;

    return match users {
        Err(_) => Err(ServerError::InternalError),
        Ok(users) => Ok(users),
    };
}

pub async fn user_uses_password(
    ctx: &Context,
    user_id: i32,
) -> Result<UsesPasswordUser, ServerError> {
    let res = User::find_by_id(user_id)
        .into_partial_model::<UsesPasswordUser>()
        .one(&ctx.db)
        .await
        .map_err(|_| ServerError::InternalError)?;

    return match res {
        None => Err(ServerError::NotFound),
        Some(user) => Ok(user),
    };
}

#[derive(Serialize)]
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

    let res = User::find_by_id(user_id)
        .one(&ctx.db)
        .await
        .map_err(|_| ServerError::InternalError)?;

    if res.is_none() {
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

pub async fn get_user(auth: &Authorization, ctx: &Context) -> Result<MainPageUser, ServerError> {
    let user = User::find_by_id(auth.sub)
        .into_partial_model::<MainPageUser>()
        .one(&ctx.db)
        .await
        .map_err(|_| ServerError::InternalError)?;

    return match user {
        None => Err(ServerError::NotFound),
        Some(user) => Ok(user),
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
