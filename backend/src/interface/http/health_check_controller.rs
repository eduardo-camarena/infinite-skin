use actix_web::{
    get,
    web::{self, Json},
    Responder,
};

use crate::service::health_check_service;

pub fn health_check_controller() -> actix_web::Scope {
    return web::scope("/").service(health_check);
}

#[get("")]
pub async fn health_check() -> impl Responder {
    let response = health_check_service::health_check();

    return Json(response);
}
