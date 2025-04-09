use crate::{
    database::{models::artist_model::PartialArtist, queries},
    Context,
};

use super::errors::server_error::ServerError;

pub async fn get_artist(ctx: Context, artist_id: i32) -> Result<PartialArtist, ServerError> {
    let res = queries::artists::find_by_id(&ctx.db, artist_id)
        .await
        .map_err(|_| ServerError::InternalError)?;

    return match res {
        None => Err(ServerError::NotFound),
        Some(artist) => Ok(artist),
    };
}
