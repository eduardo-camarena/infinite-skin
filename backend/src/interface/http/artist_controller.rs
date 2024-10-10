use crate::{service::artist_service, Context};

use actix_web::{
    get,
    web::{self, Json, Path},
    Responder,
};

pub fn artist_controller() -> actix_web::Scope {
    return web::scope("/artists").service(get_artist);
}

#[get("/{id}")]
async fn get_artist(ctx: Context, params: Path<i32>) -> impl Responder {
    let artist = artist_service::get_artist(ctx, params.into_inner()).await;

    return match artist {
        Err(err) => Err(err),
        Ok(artist) => Ok(Json(artist)),
    };
}
