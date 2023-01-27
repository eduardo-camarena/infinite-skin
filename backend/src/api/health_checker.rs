use std::time::SystemTime;

use actix_web::{get, HttpResponse, Responder};
use chrono::{Utc, DateTime};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
  pub now: String
}

#[get("/")]
pub async fn health_check() -> impl Responder {
  let system_time = SystemTime::now();
  let datetime: DateTime<Utc> = system_time.into();
  let response = &HealthCheckResponse {
    now: datetime.to_rfc2822()
  };

  HttpResponse::Ok().json(response)
}