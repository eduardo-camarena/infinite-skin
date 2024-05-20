#![allow(clippy::needless_return)]
mod database;
mod interface;
mod service;
mod utils;

use std::env;

use crate::database::db::establish_connection;
use crate::interface::http::{
    albums_controller::albums_controller, scan_controller::scan_controller,
    users_controller::users_controller,
};
use crate::utils::config::{get_config, Config};
use migration::{Migrator, MigratorTrait};

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;
use interface::http::health_check_controller::health_check_controller;
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

    let db = establish_connection().await;
    Migrator::up(&db, None).await.unwrap();

    let app_data = AppData {
        db,
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
            .service(health_check_controller())
            .service(scan_controller())
            .service(users_controller())
            .service(albums_controller())
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
