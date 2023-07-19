-- Add down migration script here
ALTER TABLE album DROP FOREIGN KEY FK_AlbumUserId;

ALTER TABLE album DROP COLUMN user_id;
ALTER TABLE album DROP COLUMN is_private;
