use std::fs::read_dir;

use actix_files::NamedFile;
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        models::album_model::{AlbumWithMetadata, PartialAlbum},
        queries,
    },
    Context,
};

use crate::service::errors::server_error::ServerError;

#[derive(Serialize)]
pub struct GetAlbumsResponse {
    pub albums: Vec<PartialAlbum>,
}

pub async fn get_albums(
    ctx: &Context,
    page_index: i32,
    artist_id: Option<i32>,
    series_id: Option<i32>,
    order_by_type: Option<String>,
    order_by_column: Option<String>,
) -> Result<GetAlbumsResponse, ServerError> {
    let albums = queries::albums::get_with_filter(
        &ctx.db,
        page_index,
        artist_id,
        series_id,
        order_by_type,
        order_by_column,
    )
    .await;

    return match albums {
        Err(_) => Err(ServerError::NotFound),
        Ok(albums) => Ok(GetAlbumsResponse { albums }),
    };
}

#[derive(Serialize, Deserialize)]
pub struct LastPageNumberResponse {
    last_page_number: u64,
}

pub async fn last_page_number(
    ctx: &Context,
    artist_id: Option<i32>,
) -> Result<LastPageNumberResponse, ServerError> {
    let album_count = queries::albums::count(&ctx.db, artist_id)
        .await
        .map_err(|_| ServerError::InternalError)?;

    return Ok(LastPageNumberResponse {
        last_page_number: (album_count as f64 / 20.0).ceil() as u64,
    });
}

pub async fn get_file(
    ctx: &Context,
    album_id: i32,
    image_id: i32,
) -> Result<NamedFile, ServerError> {
    let album = queries::albums::get_full_name(&ctx.db, album_id)
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
    let res = queries::albums::find_by_id(&ctx.db, album_id)
        .await
        .map_err(|_| ServerError::InternalError)?;

    if res.is_none() {
        return Err(ServerError::NotFound);
    }

    let album = res.unwrap();

    return Ok(AlbumWithMetadata {
        id: album.id,
        name: album.name,
        full_name: album.full_name,
        pages: album.pages,
        artist: match album.artist_id {
            Some(artist_id) => queries::artists::find_by_id(&ctx.db, artist_id)
                .await
                .map_err(|_| ServerError::InternalError)?,
            None => None,
        },
        series: match album.series_id {
            Some(series_id) => queries::series::find_by_id(&ctx.db, series_id)
                .await
                .map_err(|_| ServerError::InternalError)?,
            None => None,
        },
    });
}
