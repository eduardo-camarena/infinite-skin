use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();

    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        env::var("MYSQL_USER").expect("Database user must be set"),
        env::var("MYSQL_PASSWORD").expect("Database password must be set"),
        env::var("MYSQL_HOST").expect("Database host must be set"),
        env::var("MYSQL_PORT").expect("Database port must be set"),
        env::var("MYSQL_DATABASE").expect("Database name must be set")
    );

    println!("{}", database_url);
    let mut connection_options = ConnectOptions::new(database_url);
    connection_options
        .min_connections(5)
        .max_connections(30)
        .sqlx_logging(true);

    return Database::connect(connection_options).await.unwrap();
}
