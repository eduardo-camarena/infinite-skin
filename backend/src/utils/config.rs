use dotenv::dotenv;

use std::env;

#[derive(Clone)]
pub struct Config {
    pub image_media_folder: String,
}

pub fn get_config() -> Config {
    dotenv().ok();

    return Config {
        image_media_folder: env::var("IMAGE_MEDIA_FOLDER").expect("Media folder not set"),
    };
}
