use crate::{http::dto::albums::AlbumFiltersDTO, service::albums_service, Context};
use actix_web::{
    get,
    web::{self, Json, Path, Query},
    HttpRequest, Responder,
};

pub fn controller() -> actix_web::Scope {
    return web::scope("{library_id}/albums")
        .service(last_page_number)
        .service(get_file)
        .service(get_albums)
        .service(get_album_info);
}

#[get("/last-page-number")]
async fn last_page_number(
    ctx: Context,
    path: Path<i32>,
    query_params: Query<AlbumFiltersDTO>,
) -> impl Responder {
    let library_id = path.into_inner();
    let params = query_params.into_inner();
    let res = albums_service::last_page_number(&ctx, library_id, params.artist_id).await;

    return match res {
        Ok(album) => Ok(Json(album)),
        Err(err) => Err(err),
    };
}

#[get("/pages/{page}")]
async fn get_albums(
    path: Path<(i32, i32)>,
    query_params: Query<AlbumFiltersDTO>,
    ctx: Context,
) -> impl Responder {
    let (library_id, page_index) = path.into_inner();
    let params = query_params.into_inner();

    let res = albums_service::get_albums(
        &ctx,
        page_index - 1,
        params.artist_id,
        params.series_id,
        Some(library_id),
        params.order_by_type,
        params.order_by_column,
    )
    .await;

    return match res {
        Ok(albums) => Ok(Json(albums)),
        Err(err) => Err(err),
    };
}

#[get("/{album_id}/images/{image_id}")]
async fn get_file(req: HttpRequest, path: Path<(i32, i32, i32)>, ctx: Context) -> impl Responder {
    let (library_id, album_id, image_id) = path.into_inner();
    let res = albums_service::get_file(&ctx, album_id, library_id, image_id).await;

    return match res {
        Ok(file) => Ok(file.into_response(&req)),
        Err(err) => Err(err),
    };
}

#[get("/{album_id}")]
pub async fn get_album_info(ctx: Context, path: Path<(i32, i32)>) -> impl Responder {
    let (library_id, album_id) = path.into_inner();

    let res = albums_service::get_album_info(&ctx, album_id, library_id).await;

    return match res {
        Ok(album) => Ok(Json(album)),
        Err(err) => Err(err),
    };
}
