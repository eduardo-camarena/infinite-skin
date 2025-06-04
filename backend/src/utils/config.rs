use std::env;

#[derive(Clone)]
pub struct Config {
    pub env: String,
    pub jwt_secret: String,
}

pub fn get_config() -> Config {
    return Config {
        env: env::var("ENVIRONMENT").expect("env is not set"),
        jwt_secret: env::var("JWT_SECRET").expect("JWT secret is not set"),
    };
}
