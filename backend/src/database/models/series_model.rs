use entity::prelude::Series;
use sea_orm::{DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "Series")]
pub struct PartialSeries {
    pub id: i32,
    pub name: String,
}
