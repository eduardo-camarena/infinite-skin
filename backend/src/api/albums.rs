use actix_web::{
    get,
    web::{Data, Json, Path},
    HttpRequest, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx;
use walkdir::WalkDir;

use crate::{database::entities::album_entity::Album, AppData};

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
    let file_location = format!("{}/images/{}", app_data.config.media_folder, full_name);

    // let mut file_path = std::path::PathBuf::from(file_location);
    // let mut file = actix_files::NamedFile::open_async(&file_path).await;
    //
    // // TODO: find a better way to match different extensions that isn't a loop
    // if file.is_ok() {
    //     println!("{}", file_path.to_string_lossy());
    //     return Ok(file.unwrap().into_response(&req));
    // }
    //
    // file_path.set_extension("jpeg");
    // file = actix_files::NamedFile::open_async(&file_path).await;
    //
    // if file.is_ok() {
    //     println!("{}", file_path.to_string_lossy());
    //     return Ok(file.unwrap().into_response(&req));
    // }
    //
    // file_path.set_extension("png");
    // println!("{:?}", file_path);
    let file_path = format!(
        "{}/{}",
        &file_location,
        get_album_images(&file_location)[(image_id - 1) as usize]
    );
    println!("{}", file_path);
    let file = actix_files::NamedFile::open_async(&file_path).await;

    if file.is_ok() {
        return Ok(file.unwrap().into_response(&req));
    }

    return Err(ServerError::NotFound);
}

fn get_album_images(folder: &String) -> Vec<String> {
    let files = WalkDir::new(folder)
        .into_iter()
        .map(|file| file.ok())
        .filter(|file| file.is_some())
        .map(|file| file.unwrap())
        .filter(|file| file.metadata().unwrap().is_file())
        .map(|folder| String::from(folder.file_name().to_str().unwrap()))
        .collect::<Vec<String>>();

    return files;
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
