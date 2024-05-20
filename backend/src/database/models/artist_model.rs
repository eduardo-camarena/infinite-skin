use entity::prelude::Artist;
use sea_orm::{DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "Artist")]
pub struct PartialArtist {
    pub id: i32,
    pub name: String,
}
