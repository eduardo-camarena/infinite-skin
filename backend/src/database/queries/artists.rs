use entity::prelude::Artist;
use sea_orm::{entity::*, DbErr, EntityTrait, QueryFilter};
use sea_orm::{DatabaseConnection, InsertResult};

use crate::database::models::artist_model::PartialArtist;

pub async fn create(
    db: &DatabaseConnection,
    name: &str,
) -> Result<InsertResult<entity::artist::ActiveModel>, DbErr> {
    Artist::insert(entity::artist::ActiveModel {
        name: Set(String::from(name)),
        ..Default::default()
    })
    .exec(db)
    .await
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<PartialArtist>, DbErr> {
    Artist::find_by_id(id)
        .into_partial_model::<PartialArtist>()
        .one(db)
        .await
}

pub async fn find_by_name(
    db: &DatabaseConnection,
    name: &str,
) -> Result<Option<PartialArtist>, DbErr> {
    Artist::find()
        .filter(entity::artist::Column::Name.eq(name))
        .into_partial_model::<PartialArtist>()
        .one(db)
        .await
}
