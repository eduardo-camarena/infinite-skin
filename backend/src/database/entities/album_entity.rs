use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Album {
    pub id: i32,
    pub name: String,
    pub pages: i16,
    pub artist_id: Option<i32>,
}
