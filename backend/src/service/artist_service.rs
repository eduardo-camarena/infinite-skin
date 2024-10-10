use crate::{database::models::artist_model::PartialArtist, Context};
use entity::prelude::Artist;
use sea_orm::entity::*;

use super::errors::server_error::ServerError;

pub async fn get_artist(ctx: Context, artist_id: i32) -> Result<PartialArtist, ServerError> {
    let res = Artist::find_by_id(artist_id)
        .into_partial_model::<PartialArtist>()
        .one(&ctx.db)
        .await
        .map_err(|_| ServerError::InternalError)?;

    return match res {
        None => Err(ServerError::NotFound),
        Some(artist) => Ok(artist),
    };
}
