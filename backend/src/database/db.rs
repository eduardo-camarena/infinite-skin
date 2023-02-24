use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Could not connect to database")
}
