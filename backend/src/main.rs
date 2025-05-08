#![allow(clippy::needless_return)]
mod database;
mod http;
mod service;
mod utils;

use std::env;

use crate::database::db::establish_connection;
use crate::utils::config::{get_config, Config};
use migration::{Migrator, MigratorTrait};

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppData {
    db: DatabaseConnection,
    config: Config,
}

pub type Context = Data<AppData>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    let config = get_config();
    let db = establish_connection().await;

    if config.env == "prod" {
        Migrator::up(&db, None).await.unwrap();
    }

    let app_data = AppData { db, config };

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
            .service(http::libraries::controller())
            .service(http::users::controller())
            .service(http::albums::controller())
            .service(http::artists::controller())
            .service(http::health_check::controller())
    })
    .bind((
        "0.0.0.0",
        env::var("APP_PORT")
            .expect("app port not set")
            .parse::<u16>()
            .unwrap_or(80),
    ))?
    .run()
    .await
}
