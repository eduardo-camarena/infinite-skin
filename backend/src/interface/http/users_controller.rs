use crate::{service::users_service, utils::token::get_authorization, Context};

use actix_web::{
    get, post,
    web::{self, Json, Path},
    HttpRequest, Responder,
};
use serde::Deserialize;

pub fn users_controller() -> actix_web::Scope {
    return web::scope("/users")
        .service(login)
        .service(get_user)
        .service(get_users)
        .service(user_uses_password)
        .service(new_user);
}

#[derive(Deserialize)]
pub struct LoginPayload {
    id: i32,
    password: Option<String>,
}

#[post("/login")]
pub async fn login(ctx: Context, payload: Json<LoginPayload>) -> impl Responder {
    let res = users_service::login(&ctx, payload.id, payload.password.as_ref()).await;

    return match res {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(err),
    };
}

#[get("/me")]
pub async fn get_user(ctx: Context, req: HttpRequest) -> impl Responder {
    let auth = get_authorization(&req).unwrap();

    let res = users_service::get_user(&auth, &ctx).await;

    return match res {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(err),
    };
}

#[get("")]
pub async fn get_users(ctx: Context) -> impl Responder {
    let res = users_service::get_users(&ctx).await;

    return match res {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err(err),
    };
}

#[get("/{user_id}/uses-password")]
pub async fn user_uses_password(ctx: Context, path: Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    let res = users_service::user_uses_password(&ctx, user_id).await;

    return match res {
        Ok(uses_password) => Ok(Json(uses_password)),
        Err(err) => Err(err),
    };
}

#[derive(Deserialize)]
pub struct NewUserPayload {
    username: String,
    password: String,
    role: String,
}

#[post("/new")]
pub async fn new_user(ctx: Context, payload: Json<NewUserPayload>) -> impl Responder {
    let res =
        users_service::new_user(&ctx, &payload.username, &payload.password, &payload.role).await;

    return match res {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(err),
    };
}
