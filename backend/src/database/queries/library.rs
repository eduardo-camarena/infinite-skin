use entity::prelude::Library;
use sea_orm::{entity::*, DatabaseConnection, DbErr, EntityTrait, InsertResult};

use crate::database::models::library_model::PartialLibrary;

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    location: String,
    is_private: i8,
) -> Result<InsertResult<entity::library::ActiveModel>, DbErr> {
    Library::insert(entity::library::ActiveModel {
        name: Set(name),
        location: Set(location),
        is_private: Set(is_private),
        ..Default::default()
    })
    .exec(db)
    .await
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<PartialLibrary>, DbErr> {
    Library::find_by_id(id)
        .into_partial_model::<PartialLibrary>()
        .one(db)
        .await
}
