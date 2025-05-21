use itertools::join;
use std::collections::HashMap;
use std::fs::{self, read_dir};
use walkdir::WalkDir;

use crate::database::models::library_model::PartialLibrary;
use crate::database::models::{artist_model::PartialArtist, series_model::PartialSeries};
use crate::database::queries;
use crate::service::errors::server_error::ServerError;
use crate::Context;
use serde::Serialize;

pub async fn create(
    ctx: &Context,
    name: String,
    location: String,
    is_private: i8,
) -> Result<PartialLibrary, ServerError> {
    let folder = read_dir(&location);

    if folder.is_err() {
        return Err(ServerError::NotFound);
    }

    let res = queries::library::create(&ctx.db, name, location, is_private)
        .await
        .map_err(|_| ServerError::InternalError)?;

    Ok(queries::library::find_by_id(&ctx.db, res.last_insert_id)
        .await
        .map_err(|_| ServerError::InternalError)?
        .unwrap())
}

// tech debt: re-scan existing folders for new files
pub async fn scan(
    ctx: &Context,
    user_id: i32,
    library_ids: Option<Vec<i32>>,
) -> Result<(), ServerError> {
    let find_libraries_res = match library_ids {
        Some(library_ids) => {
            queries::library::find(
                &ctx.db,
                Some(queries::library::FindOptions::new().add_ids(library_ids)),
            )
            .await
        }
        None => queries::library::find(&ctx.db, None).await,
    };

    match find_libraries_res {
        Ok(libraries) => {
            for library in libraries {
                scan_library(
                    ctx,
                    user_id,
                    &format!("{}/{}", &ctx.config.media_folder, library.location),
                )
                .await?
            }
        }
        Err(err) => println!("There was an error while obtaining libraries: {}", err),
    }

    return Ok(());
}

pub async fn scan_library(ctx: &Context, user_id: i32, library: &str) -> Result<(), ServerError> {
    let folders = get_folders(library);
    let unpersisted_albums =
        queries::albums::get_unpersisted_albums(&ctx.db, folders, format!("{}/", library)).await;

    let albums_to_persist = get_albums_with_metadata(unpersisted_albums, library);

    let mut artists: Vec<PartialArtist> = vec![];
    let mut series: Vec<PartialSeries> = vec![];
    for album in albums_to_persist {
        let mut artist_id: Option<i32> = None;
        let mut series_id: Option<i32> = None;
        if album.artist.is_some() {
            let artist_name = album.artist.as_ref().unwrap();
            let cached_artist = artists.iter().find(|artist| artist.name == *artist_name);
            if cached_artist.is_none() {
                let persisted_artist = queries::artists::find_by_name(&ctx.db, artist_name)
                    .await
                    .map_err(|_| ServerError::InternalError)?;

                if persisted_artist.is_none() {
                    let newly_persisted_artist = queries::artists::create(&ctx.db, artist_name)
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

fn get_albums_with_metadata(folders: Vec<String>, root_folder: &str) -> Vec<AlbumWithMetadata> {
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

fn get_folders(root_folder: &str) -> Vec<String> {
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

#[derive(Serialize)]
pub struct PossibleFoldersResponse {
    folders: Vec<String>,
}

pub fn get_possible_folders(
    _ctx: &Context,
    path: &str,
) -> Result<PossibleFoldersResponse, ServerError> {
    let folders = read_dir(path)
        .unwrap()
        .filter(|folder| folder.as_ref().unwrap().metadata().unwrap().is_dir())
        .map(|folder| folder.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    Ok(PossibleFoldersResponse { folders })
}
