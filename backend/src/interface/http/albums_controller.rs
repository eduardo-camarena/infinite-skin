use crate::{service::albums_service, Context};

use actix_web::{
    get,
    web::{self, Json, Path},
    HttpRequest, Responder,
};

pub fn albums_controller() -> actix_web::Scope {
    return web::scope("/albums")
        .service(last_page_number)
        .service(get_file)
        .service(get_albums)
        .service(get_album_info);
}

#[get("/last-page-number")]
async fn last_page_number(ctx: Context) -> impl Responder {
    let conn = &ctx.pool;

    let res = albums_service::last_page_number(conn).await;

    return match res {
        Ok(album) => Ok(Json(album)),
        Err(err) => Err(err),
    };
}

#[get("/pages/{page}")]
async fn get_albums(path: Path<i32>, ctx: Context) -> impl Responder {
    let conn = &ctx.pool;
    let page_index = path.into_inner() - 1;

    let res = albums_service::get_albums(page_index, conn).await;

    return match res {
        Ok(albums) => Ok(Json(albums)),
        Err(err) => Err(err),
    };
}

#[get("/{album_id}/images/{image_id}")]
async fn get_file(req: HttpRequest, path: Path<(i32, i32)>, ctx: Context) -> impl Responder {
    let pool = &ctx.pool;
    let (album_id, image_id) = path.into_inner();
    let res = albums_service::get_file(pool, &ctx.config.media_folder, album_id, image_id).await;

    return match res {
        Ok(file) => Ok(file.into_response(&req)),
        Err(err) => Err(err),
    };
}

#[get("/{album_id}")]
pub async fn get_album_info(ctx: Context, path: Path<i32>) -> impl Responder {
    let conn = &ctx.pool;
    let album_id = path.into_inner();

    let res = albums_service::get_album_info(conn, album_id).await;

    return match res {
        Ok(album) => Ok(Json(album)),
        Err(err) => Err(err),
    };
}
