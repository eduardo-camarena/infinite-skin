use crate::{
    http::dto::libraries::{CreateLibraryDTO, ScanMediaFolderDTO},
    service::library_service,
    utils::token::get_authorization,
    Context,
};
use actix_web::{
    get, post,
    web::{self, Json, Query},
    HttpRequest, HttpResponse, Responder,
};

use super::dto::libraries::GetPossibleFoldersDTO;

pub fn controller() -> actix_web::Scope {
    return web::scope("/libraries")
        .service(get_libraries)
        .service(create_library)
        .service(get_possible_folders)
        .service(scan_media_folder);
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
    let payload = body.into_inner();
    let res = library_service::create_library(
        &ctx,
        payload.name,
        payload.location,
        payload.is_private as i8,
        authorization.unwrap().sub,
    )
    .await;

    match res {
        Ok(library) => Ok(Json(library)),
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
