use crate::{
    http::dto::libraries::{CreateLibraryDTO, ScanMediaFolderDTO},
    service::library_service,
    utils::token::get_authorization,
    Context,
};
use actix_web::{
    post,
    web::{self, Json, Query},
    HttpRequest, HttpResponse, Responder,
};

pub fn controller() -> actix_web::Scope {
    return web::scope("/library")
        .service(create)
        .service(scan_media_folder);
}

#[post("")]
async fn create(req: HttpRequest, ctx: Context, body: Json<CreateLibraryDTO>) -> impl Responder {
    let _ = get_authorization(&req);
    let payload = body.into_inner();
    let res = library_service::create(
        &ctx,
        payload.name,
        payload.location,
        payload.is_private as i8,
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
    let albums = library_service::scan(&ctx, auth.sub, params.into_inner().libraries).await;

    return match albums {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(err) => Err(err),
    };
}
