use std::fs::read_dir;

use actix_files::NamedFile;
use entity::prelude::{Album, Artist, Series};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde::{Deserialize, Serialize};

use crate::{
    database::models::{
        album_model::{AlbumWithMetadata, FullNameOnlyAlbum, PartialAlbum},
        artist_model::PartialArtist,
        series_model::PartialSeries,
    },
    Context,
};

use crate::service::errors::server_error::ServerError;

#[derive(Serialize, Deserialize)]
pub struct GetAlbumsResponse {
    id: i32,
    name: String,
}

pub async fn get_albums(ctx: &Context, page_index: i32) -> Result<Vec<PartialAlbum>, ServerError> {
    println!("{}, {}", page_index * 20, (page_index * 20) + 20);
    let albums = Album::find()
        .cursor_by(entity::album::Column::Id)
        .into_partial_model::<PartialAlbum>()
        .after(page_index * 20)
        .before((page_index * 20) + 20)
        .all(&ctx.db)
        .await;

    return match albums {
        Err(_) => Err(ServerError::NotFound),
        Ok(albums) => Ok(albums),
    };
}

#[derive(Serialize, Deserialize)]
pub struct LastPageNumberResponse {
    last_page_number: u64,
}

pub async fn last_page_number(ctx: &Context) -> Result<LastPageNumberResponse, ServerError> {
    let res = Album::find().count(&ctx.db).await;

    return match res {
        Err(_) => Err(ServerError::InternalError),
        Ok(album_count) => Ok(LastPageNumberResponse {
            last_page_number: (album_count as f64 / 20.0).ceil() as u64,
        }),
    };
}

pub async fn get_file(
    ctx: &Context,
    album_id: i32,
    image_id: i32,
) -> Result<NamedFile, ServerError> {
    let album = Album::find_by_id(album_id)
        .into_partial_model::<FullNameOnlyAlbum>()
        .one(&ctx.db)
        .await
        .map_err(|_| ServerError::InternalError)?;

    if album.is_none() {
        return Err(ServerError::ValidationError {
            field: String::from("album_id"),
        });
    }

    let album_location = format!(
        "{}/images/{}",
        ctx.config.media_folder,
        album.unwrap().full_name
    );
    let album_images = get_album_images(&album_location);
    let file = actix_files::NamedFile::open_async(&album_images[(image_id - 1) as usize]).await;

    return match file {
        Ok(file) => Ok(file),
        Err(_) => Err(ServerError::NotFound),
    };
}

fn get_album_images(folder: &String) -> Vec<String> {
    return read_dir(folder)
        .unwrap()
        .filter(|file| file.as_ref().unwrap().metadata().unwrap().is_file())
        .map(|file| file.unwrap().path().into_os_string().into_string().unwrap())
        .collect();
}

pub async fn get_album_info(
    ctx: &Context,
    album_id: i32,
) -> Result<AlbumWithMetadata, ServerError> {
    let res = Album::find_by_id(album_id)
        .one(&ctx.db)
        .await
        .map_err(|_| ServerError::InternalError)?;

    if res.is_none() {
        return Err(ServerError::NotFound);
    }

    let album = res.unwrap();

    let mut artist: Option<PartialArtist> = None;
    if album.artist_id.is_some() {
        artist = Artist::find_by_id(album.artist_id.unwrap())
            .into_partial_model::<PartialArtist>()
            .one(&ctx.db)
            .await
            .map_err(|_| ServerError::InternalError)?;
    }

    let mut series: Option<PartialSeries> = None;
    if album.series_id.is_some() {
        series = Series::find_by_id(album.series_id.unwrap())
            .into_partial_model::<PartialSeries>()
            .one(&ctx.db)
            .await
            .map_err(|_| ServerError::InternalError)?;
    }

    return Ok(AlbumWithMetadata {
        id: album.id,
        name: album.name,
        full_name: album.full_name,
        pages: album.pages,
        artist,
        series,
    });
}
