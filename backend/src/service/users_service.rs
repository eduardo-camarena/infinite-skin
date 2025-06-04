use crate::database::queries;
use crate::Context;
use crate::{
    database::models::user_model::{MainPageUser, UsesPasswordUser},
    utils::token::{create_token, Authorization},
};

use bcrypt;
use serde::Serialize;

use crate::service::errors::server_error::ServerError;

pub async fn new_user(
    ctx: &Context,
    username: &str,
    password: &str,
    role: &str,
) -> Result<LoginResponse, ServerError> {
    let config = &ctx.config;
    let hashed_password = hash_password(password);

    if hashed_password.is_err() {
        return Err(ServerError::InternalError);
    }

    let res = queries::users::create(&ctx.db, username, password, role)
        .await
        .map_err(|_| ServerError::InternalError)?;

    let new_user = queries::users::find_by_id(&ctx.db, res.last_insert_id)
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

#[derive(Serialize)]
pub struct GetUsersResponse {
    users: Vec<MainPageUser>,
}

pub async fn get_users(ctx: &Context) -> Result<GetUsersResponse, ServerError> {
    let users = queries::users::find(&ctx.db).await;

    return match users {
        Err(_) => Err(ServerError::InternalError),
        Ok(users) => Ok(GetUsersResponse { users }),
    };
}

pub async fn user_uses_password(
    ctx: &Context,
    user_id: i32,
) -> Result<UsesPasswordUser, ServerError> {
    let res = queries::users::find_user_uses_password(&ctx.db, user_id)
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
    let res = queries::users::find_by_id_with_password(&ctx.db, user_id)
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
    let user = queries::users::find_by_id(&ctx.db, auth.sub)
        .await
        .map_err(|_| ServerError::InternalError)?;

    return match user {
        None => Err(ServerError::NotFound),
        Some(user) => Ok(user),
    };
}

// the cost was not really tested, so this is probably not good for a real app,
// but also this is only going to be run by on local networks.
pub fn hash_password(password: &str) -> Result<String, ServerError> {
    return bcrypt::hash(password, 8).map_err(|_error| ServerError::InternalError);
}

pub fn verify(password: &String, hash: &str) -> Result<bool, ServerError> {
    return bcrypt::verify(password, hash).map_err(|_error| ServerError::InternalError);
}
