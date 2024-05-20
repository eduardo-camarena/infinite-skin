use entity::prelude::Album;
use sea_orm::{DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::database::models::{artist_model::PartialArtist, series_model::PartialSeries};

#[derive(Deserialize, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "Album")]
pub struct PartialAlbum {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub pages: i16,
}

#[derive(Deserialize, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "Album")]
pub struct FullNameOnlyAlbum {
    pub full_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct AlbumWithMetadata {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub pages: i16,
    pub artist: Option<PartialArtist>,
    pub series: Option<PartialSeries>,
}
