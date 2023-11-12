use dotenv::dotenv;

use std::env;

#[derive(Clone)]
pub struct Config {
    pub media_folder: String,
    pub jwt_secret: String,
}

pub fn get_config() -> Config {
    dotenv().ok();

    return Config {
        media_folder: env::var("MEDIA_FOLDER").expect("Media folder not set"),
        jwt_secret: env::var("JWT_SECRET").expect("JWT secret not set"),
    };
}
