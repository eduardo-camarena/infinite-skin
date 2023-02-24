use actix_web::{
    get,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use diesel::{QueryDsl, RunQueryDsl};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

use crate::database::{entities::album_entity::Album, schema::album};
use crate::{api::errors::user_error::UserError, AppData};

#[derive(Serialize, Deserialize)]
pub struct ImageExistsResponse {
    it_works: bool,
}

#[get("/{album}/{filename}")]
pub async fn get_file(
    req: HttpRequest,
    path: Path<(String, String)>,
) -> Result<HttpResponse, UserError> {
    dotenv().ok();

    let (album, filename) = path.into_inner();
    let mut image_media_folder: String = env::var("IMAGE_MEDIA_FOLDER")
        .expect("Media folder not set")
        .to_owned();
    image_media_folder = format!("{}/{}/{}", image_media_folder, album, filename);

    let file_path = std::path::PathBuf::from(image_media_folder);
    let file = actix_files::NamedFile::open_async(file_path).await;

    match file {
        Err(_) => Err(UserError::NotFound),
        Ok(file) => Ok(file.into_response(&req)),
    }
}

#[get("/{album_id}")]
pub async fn get_album_info(
    app_data: Data<AppData>,
    path: Path<i32>,
) -> Result<Json<Album>, UserError> {
    let mut conn = app_data.pool.get().unwrap();

    let album = album::table
        .find(path.into_inner())
        .first::<Album>(&mut conn);

    match album {
        Err(_) => Err(UserError::NotFound),
        Ok(album) => Ok(Json(album)),
    }
}
