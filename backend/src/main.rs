mod api;
mod database;
mod utils;

use std::env;

use crate::api::{albums, health_checker, users};
use crate::database::db::establish_connection;
use crate::utils::config::{get_config, Config};

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct AppData {
    pool: MySqlPool,
    config: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let pool: MySqlPool = establish_connection().await;
    let app_data = AppData {
        pool,
        config: get_config(),
    };
    HttpServer::new(move || {
        // this cors policy is obvously bad, but since this is only going to
        // run a local network I don't really care to set up a proper cors policy.
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();
        let logger = middleware::Logger::default();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(web::Data::new(app_data.clone()))
            .service(health_checker::health_check)
            .service(
                web::scope("/users")
                    .service(users::login)
                    .service(users::get_user)
                    .service(users::get_users)
                    .service(users::user_uses_password)
                    .service(users::new_user),
            )
            .service(
                web::scope("/albums")
                    .service(albums::last_page_number)
                    .service(albums::scan_media_folder)
                    .service(albums::get_file)
                    .service(albums::get_albums)
                    .service(albums::get_album_info),
            )
    })
    .bind((
        "localhost",
        env::var("APP_PORT")
            .expect("app port not set")
            .parse::<u16>()
            .unwrap_or(80),
    ))?
    .run()
    .await
}
