mod api;
mod database;

use crate::api::{health_checker, images, users};
use crate::database::db::establish_connection;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use r2d2::Pool;

#[derive(Clone)]
pub struct AppData {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = establish_connection();
    let app_data = AppData { pool };
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
                web::scope("/images")
                    .service(images::get_file)
                    .service(images::get_album_info),
            )
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
