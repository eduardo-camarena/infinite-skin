use entity::prelude::User;
use sea_orm::{entity::*, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter, Set};

use crate::database::models::user_model::{MainPageUser, UserWithPassword, UsesPasswordUser};

pub async fn create(
    db: &DatabaseConnection,
    username: &str,
    password: &str,
    role: &str,
) -> Result<InsertResult<entity::user::ActiveModel>, DbErr> {
    User::insert(entity::user::ActiveModel {
        username: Set(String::from(username)),
        password: Set(String::from(password)),
        role: Set(String::from(role)),
        ..Default::default()
    })
    .exec(db)
    .await
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<MainPageUser>, DbErr> {
    User::find_by_id(id)
        .into_partial_model::<MainPageUser>()
        .one(db)
        .await
}

pub async fn find(db: &DatabaseConnection) -> Result<Vec<MainPageUser>, DbErr> {
    User::find()
        .into_partial_model::<MainPageUser>()
        .all(db)
        .await
}

pub async fn find_user_uses_password(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<UsesPasswordUser>, DbErr> {
    User::find_by_id(id)
        .into_partial_model::<UsesPasswordUser>()
        .one(db)
        .await
}

pub async fn find_by_id_with_password(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<UserWithPassword>, DbErr> {
    User::find_by_id(id)
        .into_partial_model::<UserWithPassword>()
        .one(db)
        .await
}
