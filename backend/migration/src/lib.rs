pub use sea_orm_migration::prelude::*;

mod m20240518_204639_user_table;
mod m20240518_213309_create_album_related_tables;
mod m20240603_183946_add_tag_table;
mod m20240716_233216_add_rating_column;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240518_204639_user_table::Migration),
            Box::new(m20240518_213309_create_album_related_tables::Migration),
            Box::new(m20240603_183946_add_tag_table::Migration),
            Box::new(m20240716_233216_add_rating_column::Migration),
        ]
    }
}
