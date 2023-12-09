use std::fs::read_dir;

use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow, MySql};
use walkdir::{DirEntry, WalkDir};

use crate::database::entities::{
    album_entity::Album, artist_entity::Artist, series_entity::Series,
};
use crate::AppData;

use crate::api::errors::server_error::ServerError;

#[derive(Serialize, Deserialize)]
struct GetAlbumsResponse {
    id: i32,
    name: String,
}

#[get("/pages/{page}")]
pub async fn get_albums(path: Path<i32>, app_data: Data<AppData>) -> impl Responder {
    let conn = &app_data.pool;
    let page_index = path.into_inner() - 1;

    let albums = sqlx::query_as!(
        GetAlbumsResponse,
        "SELECT id, name FROM album ORDER BY id DESC LIMIT ?, 20",
        page_index * 20
    )
    .fetch_all(conn)
    .await;

    return match albums {
        Err(_) => Err(ServerError::NotFound),
        Ok(albums) => Ok(Json(albums)),
    };
}

#[derive(Serialize, Deserialize)]
struct LastPageNumberResponse {
    last_page_number: i32,
}

#[get("/last-page-number")]
pub async fn last_page_number(app_data: Data<AppData>) -> impl Responder {
    let conn = &app_data.pool;

    let last_page_number = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) as total FROM album")
        .fetch_one(conn)
        .await;

    return match last_page_number {
        Err(_) => Err(ServerError::InternalError),
        Ok(last_page_number) => Ok(Json(LastPageNumberResponse {
            last_page_number: (last_page_number.0 as f32 / 20.0).ceil() as i32,
        })),
    };
}

#[get("/{album_id}/images/{image_id}")]
pub async fn get_file(
    req: HttpRequest,
    path: Path<(i32, i32)>,
    app_data: Data<AppData>,
) -> impl Responder {
    let pool = &app_data.pool;
    let (album_id, image_id) = path.into_inner();

    let album =
        sqlx::query_as::<_, (String, String)>("SELECT name, full_name FROM album WHERE id=?")
            .bind(album_id)
            .fetch_one(pool)
            .await;

    if album.is_err() {
        return Err(ServerError::ValidationError {
            field: String::from("album_id"),
        });
    }

    let (name, full_name) = album.unwrap();
    let file_location = format!(
        "{}/images/{}/{} ({}).jpg",
        app_data.config.media_folder, full_name, name, image_id
    );

    let mut file_path = std::path::PathBuf::from(file_location);
    let mut file = actix_files::NamedFile::open_async(&file_path).await;

    // TODO: find a better way to match different extensions that isn't a loop
    if file.is_ok() {
        println!("{}", file_path.to_string_lossy());
        return Ok(file.unwrap().into_response(&req));
    }

    file_path.set_extension("jpeg");
    file = actix_files::NamedFile::open_async(&file_path).await;

    if file.is_ok() {
        println!("{}", file_path.to_string_lossy());
        return Ok(file.unwrap().into_response(&req));
    }

    file_path.set_extension("png");
    println!("{:?}", file_path);
    file = actix_files::NamedFile::open_async(&file_path).await;

    if file.is_ok() {
        println!("{}", file_path.to_string_lossy());
        return Ok(file.unwrap().into_response(&req));
    }

    return Err(ServerError::NotFound);
}

#[get("/{album_id}")]
pub async fn get_album_info(app_data: Data<AppData>, path: Path<i32>) -> impl Responder {
    let conn = &app_data.pool;

    let album_id = path.into_inner();

    let album = sqlx::query_as!(
        Album,
        "SELECT id, name, full_name, pages, artist_id FROM album WHERE id=?",
        album_id
    )
    .fetch_one(conn)
    .await;

    return match album {
        Err(_) => Err(ServerError::NotFound),
        Ok(album) => {
            println!("{}", album.id);
            Ok(Json(album))
        }
    };
}

#[derive(Serialize, FromRow, PartialEq, Debug)]
struct AlbumName {
    pub a: String,
}

// tech debt: re-scan folders for new files
#[post("/scan")]
pub async fn scan_media_folder(app_data: Data<AppData>) -> impl Responder {
    let media_folder = format!("{}/images", &app_data.config.media_folder);
    let pool = &app_data.pool;

    let folders = get_folders(&media_folder);
    let albums = get_albums_with_metadata(folders, &media_folder);

    let mut query_builder: sqlx::QueryBuilder<MySql> =
        sqlx::QueryBuilder::new("WITH t(a) AS (VALUES(");

    let mut separated = query_builder.separated("), (");
    for album in albums.iter() {
        separated.push_bind(&album.name);
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
        .filter(|album| names.iter().any(|n| n.a == *album.name))
        .collect::<Vec<AlbumWithMetadata>>();

    let mut artists: Vec<Artist> = vec![];
    let mut series: Vec<Series> = vec![];
    for album in albums_to_persist {
        let full_name = format!("{}/{}", &media_folder, album.full_name);
        let mut artist_id: Option<i32> = None;
        let pages = get_files(&full_name)
            .into_iter()
            .filter(|file| file.contains(".jpg") || file.contains(".jpeg") || file.contains(".png"))
            .collect::<Vec<String>>();

        if pages.len() > 0 {
            if album.artist.is_some() {
                let artist_name = String::from(album.artist.clone().unwrap());
                let cached_artist = artists.iter().find(|artist| artist.name == artist_name);
                if cached_artist.is_none() {
                    let persisted_artist =
                        sqlx::query_as::<_, Artist>("SELECT * FROM artist WHERE name=?")
                            .bind(&artist_name)
                            .fetch_one(pool)
                            .await;

                    if persisted_artist.is_err() {
                        // for some reason sqlx does not allow to return multiple columns yet.
                        let (new_artist_id,) = sqlx::query_as::<_, (i32,)>(
                            "INSERT INTO artist(name) VALUES(?) RETURNING id",
                        )
                        .bind(&artist_name)
                        .fetch_one(pool)
                        .await
                        .unwrap();

                        let new_artist = Artist {
                            id: new_artist_id,
                            name: String::from(artist_name),
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
            .bind(pages.len() as i32)
            .bind(artist_id)
            .fetch_one(pool)
            .await
            .unwrap();

            if album.series.is_some() && album.chapter_number.is_some() {
                let series_id: Option<i32>;
                let series_name = album.series.clone().unwrap();
                let cached_series = series
                    .iter()
                    .find(|collection| collection.name == *series_name);

                if cached_series.is_none() {
                    let persisted_series =
                        sqlx::query_as::<_, Series>("SELECT * FROM series WHERE name=?")
                            .bind(&series_name)
                            .fetch_one(pool)
                            .await;

                    if persisted_series.is_err() {
                        let (new_series_id,) = sqlx::query_as::<_, (i32,)>(
                            "INSERT INTO series(name) VALUES(?) RETURNING id",
                        )
                        .bind(&series_name)
                        .fetch_one(pool)
                        .await
                        .unwrap();

                        series.push(Series {
                            id: new_series_id,
                            name: String::from(series_name),
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

                sqlx::query(
                    "INSERT INTO album_series(series_id, album_id, chapter_number) VALUES(?, ?, ?)",
                )
                .bind(series_id)
                .bind(persisted_album_id)
                .bind(album.chapter_number.unwrap())
                .execute(pool)
                .await
                .expect("there was an error");
            }
        }
    }

    return Ok(HttpResponse::Ok());
}

fn get_folders(root_folder: &String) -> Vec<[String; 2]> {
    let folders = WalkDir::new(root_folder)
        .into_iter()
        .map(|file| file.ok())
        .filter(|file| file.is_some())
        .map(|file| file.unwrap())
        .filter(|file| file.metadata().unwrap().is_dir())
        .filter(|folder| folder.path().to_str().unwrap() != root_folder)
        .map(|folder| {
            return [
                String::from(folder.path().to_str().unwrap()),
                String::from(folder.file_name().to_str().unwrap()),
            ];
        })
        .collect::<Vec<[String; 2]>>();

    return folders;
}

#[derive(Debug)]
struct AlbumWithMetadata {
    name: String,
    full_name: String,
    artist: Option<String>,
    series: Option<String>,
    chapter_number: Option<i32>,
}

fn get_albums_with_metadata(
    folders: Vec<[String; 2]>,
    root_folder: &String,
) -> Vec<AlbumWithMetadata> {
    let mut albums_with_metadata: Vec<AlbumWithMetadata> = vec![];
    for [folder_path, folder] in folders {
        let (name, _) = folder.split_once(" [").unwrap_or((&folder, ""));
        let (_, full_name) = folder_path
            .split_once(format!("{}/", root_folder).as_str())
            .unwrap();

        let metadata = get_metadata(&full_name.to_string());

        let album_with_metadata = AlbumWithMetadata {
            name: String::from(name),
            full_name: full_name.to_string(),
            artist: metadata.artist,
            series: metadata.series,
            chapter_number: metadata.chapter_number,
        };

        albums_with_metadata.push(album_with_metadata);
    }
    return albums_with_metadata;
}

struct Metadata {
    artist: Option<String>,
    series: Option<String>,
    chapter_number: Option<i32>,
}

fn get_metadata(folder_path: &String) -> Metadata {
    let mut album_metadata = Metadata {
        artist: None,
        series: None,
        chapter_number: None,
    };

    for folder in folder_path.split("/") {
        let (_, metadata) = folder.split_once(" [").unwrap_or((&folder, ""));
        if metadata.len() > 0 {
            for item in metadata[..metadata.len() - 1].split(" ") {
                if item.contains("artist") {
                    album_metadata.artist = Some(String::from(item.split_once("=").unwrap().1));
                } else if item.contains("series") {
                    album_metadata.series = Some(String::from(item.split_once("=").unwrap().1));
                } else if item.contains("chapter_number") {
                    let chapter = String::from(item.split_once("=").unwrap().1).parse::<i32>();
                    if chapter.is_ok() {
                        album_metadata.chapter_number = Some(chapter.unwrap());
                    }
                }
            }
        }
    }

    return album_metadata;
}

fn get_files(folder_path: &String) -> Vec<String> {
    return read_dir(folder_path)
        .unwrap()
        .into_iter()
        .filter(|file| file.as_ref().unwrap().metadata().unwrap().is_file())
        .map(|file| file.unwrap().path().into_os_string().into_string().unwrap())
        .collect();
}
