use entity::prelude::Album;
use itertools::Itertools;
use sea_orm::{
    entity::*, ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, EntityTrait,
    InsertResult, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Statement,
};

use crate::database::models::album_model::{FullNameOnlyAlbum, PartialAlbum};

pub async fn create(
    db: &DatabaseConnection,
    name: String,
    full_name: String,
    pages: i16,
    chapter_number: i16,
    artist_id: Option<i32>,
    series_id: Option<i32>,
    user_id: i32,
) -> Result<InsertResult<entity::album::ActiveModel>, DbErr> {
    Album::insert(entity::album::ActiveModel {
        name: Set(name),
        full_name: Set(full_name),
        pages: Set(pages),
        chapter_number: Set(chapter_number),
        artist_id: Set(artist_id),
        series_id: Set(series_id),
        user_id: Set(user_id),
        ..Default::default()
    })
    .exec(db)
    .await
}

pub async fn get_unpersisted_albums(
    db: &DatabaseConnection,
    folders: Vec<String>,
    media_folder: String,
) -> Vec<String> {
    let unpersisted_albums = db.query_all(Statement::from_string(
            DatabaseBackend::MySql,
            format!(
                "WITH t(a) AS (VALUES(\"{}\")) SELECT t.a FROM t WHERE t.a NOT IN(SELECT full_name FROM album);",
                folders
                    .iter()
                    .map(|folder| folder.split_once(&media_folder).unwrap().1)
                    .join("\"), (\""),
            )
        )).await;

    unpersisted_albums
        .unwrap()
        .iter()
        .map(|row| row.try_get_many_by_index::<(String,)>())
        .map(|row| format!("{}{}", media_folder, row.unwrap().0))
        .collect_vec()
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<PartialAlbum>, DbErr> {
    Album::find_by_id(id)
        .into_partial_model::<PartialAlbum>()
        .one(db)
        .await
}

pub async fn get_full_name(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<FullNameOnlyAlbum>, DbErr> {
    Album::find_by_id(id)
        .into_partial_model::<FullNameOnlyAlbum>()
        .one(db)
        .await
}

pub async fn count(db: &DatabaseConnection, artist_id: Option<i32>) -> Result<u64, DbErr> {
    let mut query = Album::find();

    if artist_id.is_some() {
        query = query.filter(entity::album::Column::ArtistId.eq(artist_id.unwrap()));
    }

    query.count(db).await
}

pub async fn get_with_filter(
    db: &DatabaseConnection,
    page_index: i32,
    artist_id: Option<i32>,
    series_id: Option<i32>,
    order_by_type: Option<String>,
    order_by_column: Option<String>,
) -> Result<Vec<PartialAlbum>, DbErr> {
    let mut query = Album::find();

    if artist_id.is_some() {
        query = query.filter(entity::album::Column::ArtistId.eq(artist_id.unwrap()));
    }

    if series_id.is_some() {
        query = query.filter(entity::album::Column::SeriesId.eq(series_id.unwrap()));
    }

    query
        .order_by(
            get_order_by_column(order_by_column.as_ref()),
            get_order_by_type(order_by_type.as_ref()),
        )
        .offset((page_index * 20) as u64)
        .limit(20)
        .into_partial_model::<PartialAlbum>()
        .all(db)
        .await
}

fn get_order_by_column(order_by: Option<&String>) -> entity::album::Column {
    if order_by.is_none() {
        return entity::album::Column::Id;
    }

    match order_by.as_ref().unwrap().as_str() {
        "name" => entity::album::Column::Name,
        "pages" => entity::album::Column::Pages,
        "rating" => entity::album::Column::Rating,
        _ => entity::album::Column::Id,
    }
}

fn get_order_by_type(order_by_type: Option<&String>) -> Order {
    if order_by_type.is_none() {
        return Order::Desc;
    }

    match order_by_type.as_ref().unwrap().as_str() {
        "asc" => Order::Asc,
        _ => Order::Desc,
    }
}
