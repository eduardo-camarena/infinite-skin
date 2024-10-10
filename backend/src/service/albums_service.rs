use std::fs::read_dir;

use actix_files::NamedFile;
use entity::prelude::{Album, Artist, Series};
use sea_orm::{entity::*, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
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

#[derive(Deserialize, Debug)]
pub struct AlbumFilters {
    artist_id: Option<i32>,
    series_id: Option<i32>,
    order_by_type: Option<String>,
    order_by_column: Option<String>,
}

#[derive(Serialize)]
pub struct GetAlbumsResponse {
    pub albums: Vec<PartialAlbum>,
}

pub async fn get_albums(
    ctx: &Context,
    page_index: i32,
    album_filters: AlbumFilters,
) -> Result<GetAlbumsResponse, ServerError> {
    let mut query = Album::find();

    if album_filters.artist_id.is_some() {
        query = query
            .filter(entity::album::Column::ArtistId.eq(*album_filters.artist_id.as_ref().unwrap()));
    }

    if album_filters.series_id.is_some() {
        query = query
            .filter(entity::album::Column::SeriesId.eq(*album_filters.series_id.as_ref().unwrap()));
    }

    let albums = query
        .order_by(
            get_order_by_column(album_filters.order_by_column.as_ref()),
            get_order_by_type(album_filters.order_by_type.as_ref()),
        )
        .offset((page_index * 20) as u64)
        .limit(20)
        .into_partial_model::<PartialAlbum>()
        .all(&ctx.db)
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
    album_filters: AlbumFilters,
) -> Result<LastPageNumberResponse, ServerError> {
    let mut query = Album::find();

    println!("{:#?}", album_filters);
    if album_filters.artist_id.is_some() {
        query = query
            .filter(entity::album::Column::ArtistId.eq(*album_filters.artist_id.as_ref().unwrap()));
    }

    let album_count = query
        .count(&ctx.db)
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

fn get_order_by_column(order_by: Option<&String>) -> entity::album::Column {
    if order_by.is_none() {
        return entity::album::Column::Id;
    }

    match order_by.as_ref().unwrap().as_str() {
        "name" => entity::album::Column::Name,
        "pages" => entity::album::Column::Pages,
        "rating" => entity::album::Column::Rating,
        _ => entity::album::Column::Id,
    }
}

fn get_order_by_type(order_by_type: Option<&String>) -> Order {
    if order_by_type.is_none() {
        return Order::Desc;
    }

    match order_by_type.as_ref().unwrap().as_str() {
        "asc" => Order::Asc,
        _ => Order::Desc,
    }
}
