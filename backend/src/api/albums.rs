use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use dotenv::dotenv;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow, MySql, MySqlPool};
use std::{env, fmt::Error};
use walkdir::WalkDir;

use crate::database::entities::{
    album_entity::Album, artist_entity::Artist, series_entity::Series,
};
use crate::{api::errors::user_error::UserError, AppData};

use super::errors::server_error::ServerError;

#[derive(Serialize, Deserialize)]
pub struct ImageExistsResponse {
    it_works: bool,
}

#[get("/{album_id}/{filename}")]
pub async fn get_file(req: HttpRequest, path: Path<(String, String)>) -> impl Responder {
    dotenv().ok();

    let (album_id, filename) = path.into_inner();
    let mut image_media_folder = env::var("IMAGE_MEDIA_FOLDER").expect("Media folder not set");
    image_media_folder = format!("{}/{}/{}", image_media_folder, album_id, filename);

    let file_path = std::path::PathBuf::from(image_media_folder);
    let file = actix_files::NamedFile::open_async(file_path).await;

    return match file {
        Err(_) => Err(UserError::NotFound),
        Ok(file) => Ok(file.into_response(&req)),
    };
}

#[get("/{album_id}")]
pub async fn get_album_info(app_data: Data<AppData>, path: Path<i32>) -> impl Responder {
    let conn = &app_data.pool;

    let album: Result<Album, UserError> = Ok(Album {
        id: 1,
        name: String::from("hello"),
        pages: 10,
        artist_id: None,
    });

    return match album {
        Err(_) => Err(UserError::NotFound),
        Ok(album) => Ok(Json(album)),
    };
}

#[derive(Serialize, FromRow, PartialEq, Debug)]
struct AlbumName {
    pub a: String,
}

#[post("/scan")]
pub async fn scan_media_folder(app_data: Data<AppData>) -> impl Responder {
    let media_folder = env::var("IMAGE_MEDIA_FOLDER").expect("Media folder not ");
    let pool = &app_data.pool;

    let folder_names = WalkDir::new(&media_folder)
        .into_iter()
        .map(|file| file.ok())
        .filter(|file| file.is_some())
        .map(|file| file.unwrap())
        .filter(|file| file.metadata().unwrap().is_dir())
        .filter(|folder| folder.path().to_string_lossy() != media_folder)
        .map(|folder| String::from(folder.file_name().to_str().unwrap()))
        .collect::<Vec<String>>();

    let albums = get_albums_with_metadata(folder_names, pool).await;
    let album_names = albums
        .iter()
        .map(|(name, _)| String::from(name))
        .collect::<Vec<String>>();

    let mut query_builder: sqlx::QueryBuilder<MySql> =
        sqlx::QueryBuilder::new("WITH t(a) AS (VALUES(");

    let mut separated = query_builder.separated("), (");
    for album in album_names.iter() {
        separated.push_bind(album);
    }

    separated.push_unseparated(")) SELECT t.a FROM t WHERE t.a NOT IN(SELECT name FROM album)");
    let query = query_builder.build_query_as::<AlbumName>();
    let missing_albums = query.fetch_all(pool).await;

    if missing_albums.is_err() {
        return Err(ServerError::InternalError);
    }

    let names = missing_albums.unwrap();
    let albums_to_persist = albums
        .into_iter()
        .filter(|(name, _)| names.iter().any(|n| n.a == *name))
        .collect::<Vec<(String, AlbumMetadata)>>();

    for (album_name, metadata) in albums_to_persist {
        let persisted_album_id = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO album(name, pages, artist_id) VALUES(?, ?, ?) RETURNING id",
        )
        .bind(album_name)
        .bind(10)
        .bind(metadata.artist)
        .fetch_one(pool)
        .await
        .unwrap()
        .0;

        if metadata.series.is_some() {
            sqlx::query("INSERT INTO album_series(series_id, album_id) VALUES(?, ?)")
                .bind(persisted_album_id)
                .bind(metadata.series)
                .execute(pool)
                .await
                .expect("there was an error");
        }
    }

    return Ok(HttpResponse::Ok());
}

struct AlbumMetadata {
    artist: Option<i32>,
    series: Option<i32>,
}

async fn get_albums_with_metadata(
    folders: Vec<String>,
    pool: &MySqlPool,
) -> Vec<(String, AlbumMetadata)> {
    let mut albums_with_metadata: Vec<(String, AlbumMetadata)> = vec![];
    let mut artists: Vec<Artist> = vec![];
    let mut series: Vec<Series> = vec![];
    for folder in folders {
        let (name, metadata) = folder.split_once(" [").unwrap_or((&folder, ""));
        let mut album_metadata = AlbumMetadata {
            artist: None,
            series: None,
        };

        if metadata != "" {
            for item in metadata[..metadata.len() - 1].split(", ") {
                if item.contains("artist") {
                    let artist_name = item.split_once("=").unwrap().1;
                    let cached_artist = artists.iter().find(|artist| artist.name == artist_name);
                    if cached_artist.is_none() {
                        let persisted_artist =
                            sqlx::query_as::<_, Artist>("SELECT * FROM artist WHERE name=?")
                                .bind(artist_name)
                                .fetch_one(pool)
                                .await;

                        if persisted_artist.is_err() {
                            // for some reason sqlx does not allow to return multiple columns yet.
                            let new_artist_id = sqlx::query_as::<_, (i32,)>(
                                "INSERT INTO artist(name) VALUES(?) RETURNING id",
                            )
                            .bind(artist_name)
                            .fetch_one(pool)
                            .await
                            .unwrap()
                            .0;

                            let new_artist = Artist {
                                id: new_artist_id,
                                name: String::from(artist_name),
                            };
                            album_metadata.artist = Some(new_artist.id);
                            artists.push(new_artist);
                        } else {
                            let artist = persisted_artist.ok().unwrap();
                            album_metadata.artist = Some(artist.id);
                        }
                    } else {
                        album_metadata.artist = Some(cached_artist.unwrap().id);
                    }
                } else if item.contains("series") {
                    let series_name = item.split_once("=").unwrap().1;
                    let cached_series = series
                        .iter()
                        .find(|collection| collection.name == series_name);
                    if cached_series.is_none() {
                        let persisted_series =
                            sqlx::query_as::<_, Series>("SELECT * FROM artist WHERE name=?")
                                .bind(series_name)
                                .fetch_one(pool)
                                .await;

                        if persisted_series.is_err() {
                            let new_series_id = sqlx::query_as::<_, (i32,)>(
                                "INSERT INTO series(name) VALUES(?) RETURNING id",
                            )
                            .bind(series_name)
                            .fetch_one(pool)
                            .await
                            .unwrap()
                            .0;

                            album_metadata.series = Some(new_series_id);
                            series.push(Series {
                                id: new_series_id,
                                name: String::from(series_name),
                            });
                        } else {
                            album_metadata.series = Some(persisted_series.ok().unwrap().id);
                        }
                    } else {
                        album_metadata.series = Some(cached_series.unwrap().id);
                    }
                }
            }
        }
        albums_with_metadata.push((String::from(name), album_metadata));
    }
    return albums_with_metadata;
}
