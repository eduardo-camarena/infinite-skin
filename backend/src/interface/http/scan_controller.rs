use crate::{service::scan_service, utils::token::get_authorization, Context};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

pub fn scan_controller() -> actix_web::Scope {
    return web::scope("/scan").service(scan_media_folder);
}

#[post("")]
async fn scan_media_folder(req: HttpRequest, ctx: Context) -> impl Responder {
    let auth = get_authorization(&req).unwrap();
    let albums = scan_service::scan(&ctx, auth.sub).await;

    return match albums {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(err) => Err(err),
    };
}
