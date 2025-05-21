use entity::prelude::Library;
use sea_orm::{entity::*, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter};

use crate::database::models::library_model::PartialLibrary;

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    location: String,
    is_private: i8,
) -> Result<InsertResult<entity::library::ActiveModel>, DbErr> {
    Library::insert(entity::library::ActiveModel {
        name: Set(name),
        location: Set(location),
        is_private: Set(is_private),
        ..Default::default()
    })
    .exec(db)
    .await
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<PartialLibrary>, DbErr> {
    Library::find_by_id(id)
        .into_partial_model::<PartialLibrary>()
        .one(db)
        .await
}

pub struct FindOptions {
    ids: Option<Vec<i32>>,
}

impl FindOptions {
    pub fn new() -> FindOptions {
        FindOptions { ids: None }
    }

    pub fn add_ids(mut self, ids: Vec<i32>) -> Self {
        self.ids = Some(ids);
        self
    }
}

pub async fn find(
    db: &DatabaseConnection,
    find_options: Option<FindOptions>,
) -> Result<Vec<PartialLibrary>, DbErr> {
    let mut query = Library::find();

    query = match find_options {
        Some(opts) => {
            if opts.ids.is_some() {
                query = query.filter(entity::library::Column::Id.is_in(opts.ids.unwrap()))
            }

            query
        }
        None => query,
    };

    query.into_partial_model::<PartialLibrary>().all(db).await
}
