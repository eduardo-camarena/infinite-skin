use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    let mut connection_options = ConnectOptions::new(database_url);
    connection_options
        .min_connections(5)
        .max_connections(30)
        .sqlx_logging(true);

    return Database::connect(connection_options).await.unwrap();
}
