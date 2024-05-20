use entity::prelude::{Album, Artist, Series};
use itertools::Itertools;
use sea_orm::{entity::*, ConnectionTrait, DatabaseBackend, EntityTrait, QueryFilter, Statement};
use std::fs::read_dir;
use walkdir::WalkDir;

use crate::database::models::{artist_model::PartialArtist, series_model::PartialSeries};
use crate::service::errors::server_error::ServerError;
use crate::Context;

// tech debt: re-scan existing folders for new files
pub async fn scan(ctx: &Context, user_id: i32) -> Result<(), ServerError> {
    scan_albums(ctx, user_id).await?;
    scan_videos(ctx, user_id).await?;

    return Ok(());
}

pub async fn scan_videos(ctx: &Context, _user_id: i32) -> Result<(), ServerError> {
    let media_folder = format!("{}/videos", &ctx.config.media_folder);
    get_folders(&media_folder);

    return Ok(());
}

pub async fn scan_albums(ctx: &Context, user_id: i32) -> Result<(), ServerError> {
    let media_folder = format!("{}/images", &ctx.config.media_folder);
    let media_folder_with_slash = format!("{}/", &media_folder);
    let folders = get_folders(&media_folder);
    let unpersisted_albums = ctx
        .db
        .query_all(Statement::from_string(
            DatabaseBackend::MySql,
            format!(
                "WITH t(a) AS (VALUES(\"{}\")) SELECT t.a FROM t WHERE t.a NOT IN(SELECT full_name FROM album);",
                folders
        .iter()
        .map(|folder| folder.split_once(&media_folder_with_slash).unwrap().1)
        .join("\"), (\"")
            )
        ))
        .await;

    let albums_to_persist = get_albums_with_metadata(
        unpersisted_albums
            .unwrap()
            .iter()
            .map(|row| row.try_get_many_by_index::<(String,)>())
            .map(|row| format!("{}{}", media_folder_with_slash, row.unwrap().0))
            .collect_vec(),
        &media_folder,
    );

    let mut artists: Vec<PartialArtist> = vec![];
    let mut series: Vec<PartialSeries> = vec![];
    for album in albums_to_persist {
        let mut artist_id: Option<i32> = None;
        let mut series_id: Option<i32> = None;
        if album.artist.is_some() {
            let artist_name = album.artist.as_ref().unwrap();
            let cached_artist = artists.iter().find(|artist| artist.name == *artist_name);
            if cached_artist.is_none() {
                let persisted_artist = Artist::find()
                    .filter(entity::artist::Column::Name.eq(artist_name))
                    .into_partial_model::<PartialArtist>()
                    .one(&ctx.db)
                    .await
                    .map_err(|_| ServerError::InternalError)?;

                if persisted_artist.is_none() {
                    let newly_persisted_artist = Artist::insert(entity::artist::ActiveModel {
                        name: Set(String::from(artist_name)),
                        ..Default::default()
                    })
                    .exec(&ctx.db)
                    .await
                    .map_err(|_| ServerError::InternalError)?;

                    let new_artist = PartialArtist {
                        id: newly_persisted_artist.last_insert_id,
                        name: String::from(artist_name),
                    };
                    artist_id = Some(new_artist.id);
                    artists.push(new_artist);
                } else {
                    artist_id = Some(persisted_artist.as_ref().unwrap().id);
                    artists.push(persisted_artist.unwrap());
                }
            } else {
                artist_id = Some(cached_artist.unwrap().id);
            }
        }

        if album.series.is_some() {
            let series_name = album.series.as_ref().unwrap();
            let cached_series = series
                .iter()
                .find(|collection| collection.name == *series_name);

            if cached_series.is_none() {
                let persisted_series = Series::find()
                    .filter(entity::series::Column::Name.eq(series_name))
                    .into_partial_model::<PartialSeries>()
                    .one(&ctx.db)
                    .await
                    .map_err(|_| ServerError::InternalError)?;

                if persisted_series.is_none() {
                    let newly_persisted_series = Series::insert(entity::series::ActiveModel {
                        name: Set(String::from(series_name)),
                        ..Default::default()
                    })
                    .exec(&ctx.db)
                    .await
                    .map_err(|_| ServerError::InternalError)?;

                    let new_series = PartialSeries {
                        id: newly_persisted_series.last_insert_id,
                        name: String::from(series_name),
                    };

                    series_id = Some(new_series.id);
                    series.push(new_series);
                } else {
                    let collection = persisted_series.unwrap();
                    series_id = Some(collection.id);
                    series.push(collection);
                }
            } else {
                series_id = Some(cached_series.unwrap().id);
            }
        }

        let mut chapter_number: i16 = 1;
        if album.chapter_number.is_some() {
            chapter_number = album.chapter_number.unwrap();
        }

        println!("{:?}{:?}", artist_id, series_id);
        Album::insert(entity::album::ActiveModel {
            name: Set(album.name),
            full_name: Set(album.full_name),
            pages: Set(album.pages.len() as i16),
            chapter_number: Set(chapter_number),
            artist_id: Set(artist_id),
            series_id: Set(series_id),
            user_id: Set(user_id),
            ..Default::default()
        })
        .exec(&ctx.db)
        .await
        .map_err(|_| ServerError::InternalError)?;
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
    chapter_number: Option<i16>,
}

fn get_albums_with_metadata(folders: Vec<String>, root_folder: &String) -> Vec<AlbumWithMetadata> {
    let mut albums_with_metadata: Vec<AlbumWithMetadata> = vec![];
    for folder_path in folders {
        let (_, full_name) = folder_path
            .split_once(format!("{}/", root_folder).as_str())
            .unwrap();
        let name = full_name
            .split('/')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_owned();

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
                    let chapter = String::from(item.split_once('=').unwrap().1).parse::<i16>();
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
