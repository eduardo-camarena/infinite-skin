mod api;

use api::health_checker;

use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_checker::health_check)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}