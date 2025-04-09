use entity::prelude::Series;
use sea_orm::{entity::*, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter};

use crate::database::models::series_model::PartialSeries;

pub async fn create(
    db: &DatabaseConnection,
    name: &str,
) -> Result<InsertResult<entity::series::ActiveModel>, DbErr> {
    Series::insert(entity::series::ActiveModel {
        name: Set(String::from(name)),
        ..Default::default()
    })
    .exec(db)
    .await
}

pub async fn get_by_name(
    db: &DatabaseConnection,
    name: &str,
) -> Result<Option<PartialSeries>, DbErr> {
    Series::find()
        .filter(entity::series::Column::Name.eq(name))
        .into_partial_model::<PartialSeries>()
        .one(db)
        .await
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<PartialSeries>, DbErr> {
    Series::find()
        .filter(entity::series::Column::Id.eq(id))
        .into_partial_model::<PartialSeries>()
        .one(db)
        .await
}
