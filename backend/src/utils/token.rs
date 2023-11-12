use std::env;

use actix_web::{http::StatusCode, HttpRequest};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    iat: i64,
    exp: i64,
}

pub fn create_token(sub: i32, jwt_secret: &String) -> Result<String, StatusCode> {
    let iat = chrono::offset::Utc::now().timestamp_millis() / 100;

    let exp = iat + chrono::Duration::weeks(52).num_seconds();
    return match encode(
        &Header::default(),
        &Claims { sub, iat, exp },
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(jwt) => Ok(jwt),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

pub fn get_authorization(request: &HttpRequest) -> Result<Claims, StatusCode> {
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let auth = request.headers().get("Authorization");

    if auth.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth
        .unwrap()
        .clone()
        .to_str()
        .unwrap()
        .replace("Bearer ", "");

    return match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(decoded_token) => Ok(decoded_token.claims),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    };
}
