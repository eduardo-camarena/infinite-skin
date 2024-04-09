use crate::{service::scan_service, Context};
use actix_web::{post, web, HttpResponse, Responder};

pub fn scan_controller() -> actix_web::Scope {
    return web::scope("/scan").service(scan_media_folder);
}

#[post("")]
async fn scan_media_folder(ctx: Context) -> impl Responder {
    let albums = scan_service::scan_albums(&ctx).await;

    return match albums {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(err) => Err(err),
    };
}
