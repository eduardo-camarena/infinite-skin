use dotenvy::dotenv;

use std::env;

#[derive(Clone)]
pub struct Config {
    pub env: String,
    pub media_folder: String,
    pub jwt_secret: String,
}

pub fn get_config() -> Config {
    dotenv().ok();

    return Config {
        env: env::var("ENV").expect("env is not set"),
        media_folder: env::var("MEDIA_FOLDER").expect("media folder is not set"),
        jwt_secret: env::var("JWT_SECRET").expect("JWT secret is not set"),
    };
}
