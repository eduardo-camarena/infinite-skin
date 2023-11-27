use dotenvy::dotenv;

use std::env;

#[derive(Clone)]
pub struct Config {
    pub media_folder: String,
    pub jwt_secret: String,
}

pub fn get_config() -> Config {
    dotenv().ok();

    return Config {
        media_folder: "/media_folder".to_string(),
        jwt_secret: env::var("JWT_SECRET").expect("JWT secret not set"),
    };
}
