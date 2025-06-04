use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE artist(
                id INT AUTO_INCREMENT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )",
        )
        .await?;

        db.execute_unprepared(
            "CREATE TABLE series(
                id INT AUTO_INCREMENT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )",
        )
        .await?;

        db.execute_unprepared(
            "CREATE TABLE library(
                id INT AUTO_INCREMENT PRIMARY KEY,
                name TEXT NOT NULL,
                location TEXT NOT NULL,
                is_private BOOLEAN NOT NULL DEFAULT FALSE,
                user_id INT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE NO ACTION ON UPDATE NO ACTION
            )",
        )
        .await?;

        db.execute_unprepared(
            "CREATE TABLE album(
                id INT AUTO_INCREMENT PRIMARY KEY,
                name TEXT NOT NULL,
                full_name TEXT NOT NULL,
                pages SMALLINT NOT NULL,
                is_private BOOLEAN NOT NULL DEFAULT FALSE,
                chapter_number SMALLINT NOT NULL DEFAULT 0,
                series_id INT,
                artist_id INT,
                library_id INT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                FOREIGN KEY (artist_id) REFERENCES artist(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
                FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
                FOREIGN KEY (library_id) REFERENCES library(id) ON DELETE NO ACTION ON UPDATE NO ACTION
            )",
        )
        .await?;

        db.execute_unprepared(
            "CREATE TABLE album_info(
                id INT AUTO_INCREMENT PRIMARY KEY,
                description TEXT NOT NULL,
                rating TINYINT NOT NULL CHECK (rating >= 0 AND rating <= 10),
                album_id INT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                FOREIGN KEY (album_id) REFERENCES album(id) ON DELETE NO ACTION ON UPDATE NO ACTION
            )",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE album_info").await?;
        db.execute_unprepared("DROP TABLE album").await?;
        db.execute_unprepared("DROP TABLE library").await?;
        db.execute_unprepared("DROP TABLE series").await?;
        db.execute_unprepared("DROP TABLE artist").await?;
        Ok(())
    }
}
