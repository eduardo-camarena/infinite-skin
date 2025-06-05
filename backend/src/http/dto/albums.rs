use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AlbumFiltersDTO {
    pub artist_id: Option<i32>,
    pub series_id: Option<i32>,
    pub order_by_type: Option<String>,
    pub order_by_column: Option<String>,
}
