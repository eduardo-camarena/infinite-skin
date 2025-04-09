use itertools::join;
use std::collections::HashMap;
use std::fs::{self, read_dir};
use walkdir::WalkDir;

use crate::database::models::{artist_model::PartialArtist, series_model::PartialSeries};
use crate::database::queries;
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
    let folders = get_folders(&media_folder);
    let unpersisted_albums =
        queries::albums::get_unpersisted_albums(&ctx.db, folders, format!("{}/", &media_folder))
            .await;

    let albums_to_persist = get_albums_with_metadata(unpersisted_albums, &media_folder);

    let mut artists: Vec<PartialArtist> = vec![];
    let mut series: Vec<PartialSeries> = vec![];
    for album in albums_to_persist {
        let mut artist_id: Option<i32> = None;
        let mut series_id: Option<i32> = None;
        if album.artist.is_some() {
            let artist_name = album.artist.as_ref().unwrap();
            let cached_artist = artists.iter().find(|artist| artist.name == *artist_name);
            if cached_artist.is_none() {
                let persisted_artist = queries::artists::find_by_name(&ctx.db, &artist_name)
                    .await
                    .map_err(|_| ServerError::InternalError)?;

                if persisted_artist.is_none() {
                    let newly_persisted_artist = queries::artists::create(&ctx.db, &artist_name)
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
                let persisted_series = queries::series::get_by_name(&ctx.db, series_name)
                    .await
                    .map_err(|_| ServerError::InternalError)?;

                if persisted_series.is_none() {
                    let newly_persisted_series = queries::series::create(&ctx.db, series_name)
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

        queries::albums::create(
            &ctx.db,
            album.name,
            album.full_name,
            album.pages.len() as i16,
            chapter_number,
            artist_id,
            series_id,
            user_id,
        )
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
            let metadata = get_metadata(full_name, root_folder);
            let album_with_metadata = AlbumWithMetadata {
                name: String::from(name),
                full_name: full_name.to_string(),
                pages,
                artist: metadata.get("artist").map(String::from),
                series: metadata.get("series").map(String::from),
                chapter_number: metadata
                    .get("chapter_number")
                    .map(|val| val.parse::<i16>().unwrap_or(1)),
            };

            albums_with_metadata.push(album_with_metadata);
        }
    }

    return albums_with_metadata;
}

fn get_metadata(folder_path: &str, root_folder: &str) -> HashMap<String, String> {
    let mut metadata: HashMap<String, String> = HashMap::new();
    let folders = folder_path.split("/").collect::<Vec<&str>>();
    let mut folders_to_use = folders.len() - 1;

    while folders_to_use > 0 {
        let file_name = format!(
            "{}/{}/metadata.txt",
            root_folder,
            join(&folders[0..folders_to_use], "/")
        );
        let metadata_file = fs::read_to_string(file_name);
        if metadata_file.is_ok() {
            for line in metadata_file.unwrap().lines() {
                let (key, value) = line.split_once("=").unwrap_or(("", ""));
                metadata.insert(String::from(key), String::from(value));
            }
        }
        folders_to_use -= 1;
    }

    metadata
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
