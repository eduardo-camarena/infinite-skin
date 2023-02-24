use crate::database::schema::user;

use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct InsertableUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}
