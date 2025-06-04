use sea_orm::{DerivePartialModel, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::prelude::Library")]
pub struct PartialLibrary {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub location: String,
}
