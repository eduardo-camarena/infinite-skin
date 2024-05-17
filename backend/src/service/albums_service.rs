use std::fs::read_dir;

use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use sqlx::{self, MySqlPool};

use crate::{database::entities::album_entity::Album, Context};

use crate::service::errors::server_error::ServerError;

#[derive(Serialize, Deserialize)]
pub struct GetAlbumsResponse {
    id: i32,
    name: String,
}

pub async fn get_albums(
    ctx: &Context,
    page_index: i32,
) -> Result<Vec<GetAlbumsResponse>, ServerError> {
    let albums = sqlx::query_as!(
        GetAlbumsResponse,
        "SELECT id, name FROM album ORDER BY id DESC LIMIT ?, 20",
        page_index * 20
    )
    .fetch_all(&ctx.pool)
    .await;

    return match albums {
        Err(_) => Err(ServerError::NotFound),
        Ok(albums) => Ok(albums),
    };
}

#[derive(Serialize, Deserialize)]
pub struct LastPageNumberResponse {
    last_page_number: i32,
}

pub async fn last_page_number(ctx: &Context) -> Result<LastPageNumberResponse, ServerError> {
    let last_page_number = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) as total FROM album")
        .fetch_one(&ctx.pool)
        .await;

    return match last_page_number {
        Err(_) => Err(ServerError::InternalError),
        Ok(last_page_number) => Ok(LastPageNumberResponse {
            last_page_number: (last_page_number.0 as f32 / 20.0).ceil() as i32,
        }),
    };
}

pub async fn get_file(
    ctx: &Context,
    album_id: i32,
    image_id: i32,
) -> Result<NamedFile, ServerError> {
    let full_name = sqlx::query_as::<_, (String,)>("SELECT full_name FROM album WHERE id=?")
        .bind(album_id)
        .fetch_one(&ctx.pool)
        .await;

    if full_name.is_err() {
        return Err(ServerError::ValidationError {
            field: String::from("album_id"),
        });
    }

    let album_location = format!("{}/image/{}", ctx.config.media_folder, full_name.unwrap().0);
    println!("{}", album_location);
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

pub async fn get_album_info(ctx: &Context, album_id: i32) -> Result<Album, ServerError> {
    let album = sqlx::query_as!(
        Album,
        "SELECT id, name, full_name, pages, artist_id FROM album WHERE id=?",
        album_id
    )
    .fetch_one(&ctx.pool)
    .await;

    return match album {
        Ok(album) => Ok(album),
        Err(_) => Err(ServerError::NotFound),
    };
}
