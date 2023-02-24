use crate::database::entities::user_entity::User;
use crate::database::schema::user;
use crate::AppData;

use actix_web::{
    get,
    web::{Data, Json},
};
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserExists {
    it_works: bool,
}

#[get("/")]
pub async fn get_user(app_data: Data<AppData>) -> Json<User> {
    let mut connection = app_data.pool.get().unwrap();

    let found_user = user::table
        .find(1)
        .first::<User>(&mut connection)
        .expect("There was an error while loading the user");

    Json(found_user)
}
