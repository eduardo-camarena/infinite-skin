use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub now: String,
}

pub fn health_check() -> HealthCheckResponse {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();

    return HealthCheckResponse {
        now: datetime.to_rfc2822(),
    };
}
