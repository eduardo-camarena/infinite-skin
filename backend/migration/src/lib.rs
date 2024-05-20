pub use sea_orm_migration::prelude::*;

mod m20240518_204639_user_table;
mod m20240518_213309_create_album_related_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240518_204639_user_table::Migration),
            Box::new(m20240518_213309_create_album_related_tables::Migration),
        ]
    }
}
