use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

pub async fn establish_connection() -> MySqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    return MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .unwrap();
}
