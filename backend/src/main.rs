mod api;

use api::{health_checker};

use actix_web::{App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        let logger = middleware::Logger::default();
        App::new()
            .wrap(logger)
            .service(health_checker::health_check)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
