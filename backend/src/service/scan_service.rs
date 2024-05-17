use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::{self, MySql};
use std::fs::read_dir;
use walkdir::WalkDir;

use crate::database::entities::{artist_entity::Artist, series_entity::Series};
use crate::service::errors::server_error::ServerError;
use crate::Context;

#[derive(Serialize, FromRow, PartialEq, Debug)]
struct AlbumName {
    pub a: String,
}

// tech debt: re-scan existing folders for new files
pub async fn scan(ctx: &Context) -> Result<(), ServerError> {
    scan_albums(ctx).await?;
    scan_videos(ctx).await?;

    return Ok(());
}

pub async fn scan_videos(ctx: &Context) -> Result<(), ServerError> {
    let media_folder = format!("{}/videos", &ctx.config.media_folder);
    get_folders(&media_folder);

    return Ok(());
}

pub async fn scan_albums(ctx: &Context) -> Result<(), ServerError> {
    let media_folder = format!("{}/images", &ctx.config.media_folder);
    let folders = get_folders(&media_folder);
    let albums = get_albums_with_metadata(folders, &media_folder);

    let mut query_builder = sqlx::QueryBuilder::<MySql>::new("WITH t(a) AS (VALUES(");

    let mut separated = query_builder.separated("), (");
    for album in albums.iter() {
        separated.push_bind(&album.name);
    }

    separated.push_unseparated(")) SELECT t.a FROM t WHERE t.a NOT IN(SELECT name FROM album)");
    let missing_albums = query_builder
        .build_query_as::<AlbumName>()
        .fetch_all(&ctx.pool)
        .await;

    if missing_albums.is_err() {
        return Err(ServerError::InternalError);
    }

    let names = missing_albums.unwrap();
    let albums_to_persist = albums
        .into_iter()
        .filter(|album| names.iter().any(|n| n.a == *album.name))
        .collect::<Vec<AlbumWithMetadata>>();

    let mut artists: Vec<Artist> = vec![];
    let mut series: Vec<Series> = vec![];
    for album in albums_to_persist {
        let mut artist_id: Option<i32> = None;
        if album.artist.is_some() {
            let artist_name = album.artist.clone().unwrap();
            let cached_artist = artists.iter().find(|artist| artist.name == artist_name);
            if cached_artist.is_none() {
                let persisted_artist =
                    sqlx::query_as::<_, Artist>("SELECT * FROM artist WHERE name=?")
                        .bind(&artist_name)
                        .fetch_one(&ctx.pool)
                        .await;

                if persisted_artist.is_err() {
                    // for some reason sqlx does not allow to return multiple columns yet.
                    let (new_artist_id,) = sqlx::query_as::<_, (i32,)>(
                        "INSERT INTO artist(name) VALUES(?) RETURNING id",
                    )
                    .bind(&artist_name)
                    .fetch_one(&ctx.pool)
                    .await
                    .unwrap();

                    let new_artist = Artist {
                        id: new_artist_id,
                        name: artist_name,
                    };
                    artist_id = Some(new_artist.id);
                    artists.push(new_artist);
                } else {
                    let artist = persisted_artist.ok().unwrap();
                    artist_id = Some(artist.id);
                    artists.push(artist);
                }
            } else {
                artist_id = Some(cached_artist.unwrap().id);
            }
        }

        let (persisted_album_id,) = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO album(name, full_name, pages, artist_id) VALUES(?, ?, ?, ?) RETURNING id",
        )
        .bind(album.name)
        .bind(album.full_name)
        .bind(album.pages.len() as i32)
        .bind(artist_id)
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        if album.series.is_some() {
            let series_id: Option<i32>;
            let series_name = album.series.clone().unwrap();
            let cached_series = series
                .iter()
                .find(|collection| collection.name == *series_name);

            if cached_series.is_none() {
                let persisted_series =
                    sqlx::query_as::<_, Series>("SELECT * FROM series WHERE name=?")
                        .bind(&series_name)
                        .fetch_one(&ctx.pool)
                        .await;

                if persisted_series.is_err() {
                    let (new_series_id,) = sqlx::query_as::<_, (i32,)>(
                        "INSERT INTO series(name) VALUES(?) RETURNING id",
                    )
                    .bind(&series_name)
                    .fetch_one(&ctx.pool)
                    .await
                    .unwrap();

                    series.push(Series {
                        id: new_series_id,
                        name: series_name,
                    });
                    series_id = Some(new_series_id);
                } else {
                    let collection = persisted_series.unwrap();
                    series_id = Some(collection.id);
                    series.push(collection);
                }
            } else {
                series_id = Some(cached_series.unwrap().id);
            }

            let mut chapter_number = 1;
            if album.chapter_number.is_some() {
                chapter_number = album.chapter_number.unwrap();
            }

            let res = sqlx::query(
                "INSERT INTO album_series(series_id, album_id, chapter_number) VALUES(?, ?, ?)",
            )
            .bind(series_id)
            .bind(persisted_album_id)
            .bind(chapter_number)
            .execute(&ctx.pool)
            .await;

            if res.is_err() {
                let (amount_of_chapters,) = sqlx::query_as::<_, (i32,)>(
                    "SELECT COUNT(*) FROM album_series WHERE series_id=?",
                )
                .bind(series_id)
                .fetch_one(&ctx.pool)
                .await
                .expect("there was an error");

                sqlx::query(
                    "INSERT INTO album_series(series_id, album_id, chapter_number) VALUES(?, ?, ?)",
                )
                .bind(series_id)
                .bind(persisted_album_id)
                .bind(amount_of_chapters + 1)
                .execute(&ctx.pool)
                .await
                .expect("there was an error");
            }
        }
    }

    return Ok(());
}

#[derive(Debug)]
struct AlbumWithMetadata {
    name: String,
    full_name: String,
    pages: Vec<String>,
    artist: Option<String>,
    series: Option<String>,
    chapter_number: Option<i32>,
}

fn get_albums_with_metadata(folders: Vec<String>, root_folder: &String) -> Vec<AlbumWithMetadata> {
    let mut albums_with_metadata: Vec<AlbumWithMetadata> = vec![];
    for folder_path in folders {
        let (_, full_name) = folder_path
            .split_once(format!("{}/", root_folder).as_str())
            .unwrap();
        let (name, _) = full_name.split_once(" [").unwrap_or((full_name, ""));

        let pages = get_files(&folder_path)
            .into_iter()
            .filter(|file| file.contains(".jpg") || file.contains(".jpeg") || file.contains(".png"))
            .collect::<Vec<String>>();

        if !pages.is_empty() {
            let mut album_with_metadata = AlbumWithMetadata {
                name: String::from(name),
                full_name: full_name.to_string(),
                pages,
                artist: None,
                series: None,
                chapter_number: None,
            };
            add_metadata(full_name, &mut album_with_metadata);

            albums_with_metadata.push(album_with_metadata);
        }
    }

    return albums_with_metadata;
}

fn add_metadata(folder_path: &str, album_metadata: &mut AlbumWithMetadata) {
    for folder in folder_path.split('/') {
        let (_, metadata) = folder.split_once('[').unwrap_or((folder, ""));
        if !metadata.is_empty() {
            for item in metadata[..metadata.len() - 1].split(", ") {
                if item.contains("artist") {
                    album_metadata.artist = Some(String::from(item.split_once('=').unwrap().1));
                } else if item.contains("series") {
                    album_metadata.series = Some(String::from(item.split_once('=').unwrap().1));
                } else if item.contains("chapter_number") {
                    let chapter = String::from(item.split_once('=').unwrap().1).parse::<i32>();
                    if chapter.is_ok() {
                        album_metadata.chapter_number = Some(chapter.unwrap());
                    }
                }
            }
        }
    }
}

fn get_folders(root_folder: &String) -> Vec<String> {
    return WalkDir::new(root_folder)
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|file| file.metadata().unwrap().is_dir())
        .filter(|folder| folder.path().to_str().unwrap() != root_folder)
        .map(|folder| folder.path().to_string_lossy().to_string())
        .collect();
}

fn get_files(folder_path: &String) -> Vec<String> {
    return read_dir(folder_path)
        .unwrap()
        .filter(|file| file.as_ref().unwrap().metadata().unwrap().is_file())
        .map(|file| file.unwrap().path().into_os_string().into_string().unwrap())
        .collect();
}
