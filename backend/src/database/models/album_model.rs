use entity::prelude::Album;
use sea_orm::{prelude::DateTimeUtc, DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::database::models::{artist_model::PartialArtist, series_model::PartialSeries};

#[derive(Deserialize, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "Album")]
pub struct PartialAlbum {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub pages: i16,
    pub artist_id: Option<i32>,
    pub series_id: Option<i32>,
    pub created_at: DateTimeUtc,
}

#[derive(Deserialize, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "Album")]
pub struct ObtainLocationAlbum {
    pub library_id: i32,
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
