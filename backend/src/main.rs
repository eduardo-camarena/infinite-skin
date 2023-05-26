mod api;
mod database;
mod utils;

use crate::api::{albums, health_checker, users};
use crate::database::db::establish_connection;
use crate::utils::config::{get_config, Config};

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
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

    let pool: MySqlPool = establish_connection().await;
    let app_data = AppData {
        pool,
        config: get_config(),
    };
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();
        let logger = middleware::Logger::default();

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(web::Data::new(app_data.clone()))
            .service(health_checker::health_check)
            .service(web::scope("/users").service(users::get_user))
            .service(
                web::scope("/albums")
                    .service(albums::get_file)
                    .service(albums::get_album_info)
                    .service(albums::scan_media_folder),
            )
    })
    .bind(("localhost", 8001))?
    .run()
    .await
}
