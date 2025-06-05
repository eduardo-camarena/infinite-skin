use crate::{
    http::dto::{
        albums::AlbumFiltersDTO,
        libraries::{CreateLibraryDTO, ScanMediaFolderDTO},
    },
    service::{albums_service, library_service},
    utils::token::get_authorization,
    Context,
};
use actix_web::{
    get, post,
    web::{self, Json, Path, Query},
    HttpRequest, HttpResponse, Responder,
};

use super::{albums, dto::libraries::GetPossibleFoldersDTO};

pub fn controller() -> actix_web::Scope {
    return web::scope("/libraries")
        .service(get_libraries)
        .service(create_library)
        .service(get_possible_folders)
        .service(scan_media_folder)
        .service(albums::controller());
}

#[get("")]
async fn get_libraries(req: HttpRequest, ctx: Context) -> impl Responder {
    let _ = get_authorization(&req);
    let res = library_service::get_libraries(&ctx).await;
    match res {
        Ok(libraries) => Ok(Json(libraries)),
        Err(err) => Err(err),
    }
}

#[post("")]
async fn create_library(
    req: HttpRequest,
    ctx: Context,
    body: Json<CreateLibraryDTO>,
) -> impl Responder {
    let authorization = get_authorization(&req);
    let user_id = authorization.unwrap().sub;
    let payload = body.into_inner();
    let res = library_service::create_library(
        &ctx,
        payload.name,
        payload.location,
        payload.is_private as i8,
        user_id,
    )
    .await;

    match res {
        Ok(library) => {
            let scan_res = library_service::scan(&ctx, user_id, Some(vec![library.id])).await;

            if scan_res.is_err() {
                println!("there was an error while scanning library");
            }

            Ok(Json(library))
        }
        Err(err) => Err(err),
    }
}

#[post("/scan")]
async fn scan_media_folder(
    req: HttpRequest,
    ctx: Context,
    params: Query<ScanMediaFolderDTO>,
) -> impl Responder {
    let auth = get_authorization(&req).unwrap();
    let albums = library_service::scan(&ctx, auth.sub, params.into_inner().library_ids).await;

    return match albums {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(err) => Err(err),
    };
}

#[get("/possible-folders")]
async fn get_possible_folders(
    req: HttpRequest,
    ctx: Context,
    params: Query<GetPossibleFoldersDTO>,
) -> impl Responder {
    let _ = get_authorization(&req).unwrap();
    let payload = params.into_inner();

    let res = library_service::get_possible_folders(&ctx, &payload.path);

    match res {
        Ok(possible_folders) => Ok(Json(possible_folders)),
        Err(err) => Err(err),
    }
}
