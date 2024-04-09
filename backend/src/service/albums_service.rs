use std::fs::read_dir;

use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use sqlx::{self, MySqlPool};

use crate::database::entities::album_entity::Album;

use crate::service::errors::server_error::ServerError;

#[derive(Serialize, Deserialize)]
pub struct GetAlbumsResponse {
    id: i32,
    name: String,
}

pub async fn get_albums(
    page_index: i32,
    conn: &MySqlPool,
) -> Result<Vec<GetAlbumsResponse>, ServerError> {
    if true {
        return Err(ServerError::Unauthorized);
    }
    let albums = sqlx::query_as!(
        GetAlbumsResponse,
        "SELECT id, name FROM album ORDER BY id DESC LIMIT ?, 20",
        page_index * 20
    )
    .fetch_all(conn)
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

pub async fn last_page_number(conn: &MySqlPool) -> Result<LastPageNumberResponse, ServerError> {
    let last_page_number = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) as total FROM album")
        .fetch_one(conn)
        .await;

    return match last_page_number {
        Err(_) => Err(ServerError::InternalError),
        Ok(last_page_number) => Ok(LastPageNumberResponse {
            last_page_number: (last_page_number.0 as f32 / 20.0).ceil() as i32,
        }),
    };
}

pub async fn get_file(
    conn: &MySqlPool,
    media_folder: &String,
    album_id: i32,
    image_id: i32,
) -> Result<NamedFile, ServerError> {
    let full_name = sqlx::query_as::<_, (String,)>("SELECT full_name FROM album WHERE id=?")
        .bind(album_id)
        .fetch_one(conn)
        .await;

    if full_name.is_err() {
        return Err(ServerError::ValidationError {
            field: String::from("album_id"),
        });
    }

    let file_location = format!("{}/images/{}", media_folder, full_name.unwrap().0);
    let album_images = get_album_images(&file_location);
    let file = actix_files::NamedFile::open_async(&album_images[(image_id - 1) as usize]).await;

    return match file {
        Ok(file) => Ok(file),
        Err(_) => Err(ServerError::NotFound),
    };
}

fn get_album_images(folder: &String) -> Vec<String> {
    return read_dir(folder)
        .unwrap()
        .into_iter()
        .filter(|file| file.as_ref().unwrap().metadata().unwrap().is_file())
        .map(|file| file.unwrap().path().into_os_string().into_string().unwrap())
        .collect();
}

pub async fn get_album_info(conn: &MySqlPool, album_id: i32) -> Result<Album, ServerError> {
    let album = sqlx::query_as!(
        Album,
        "SELECT id, name, full_name, pages, artist_id FROM album WHERE id=?",
        album_id
    )
    .fetch_one(conn)
    .await;

    return match album {
        Ok(album) => Ok(album),
        Err(_) => Err(ServerError::NotFound),
    };
}
