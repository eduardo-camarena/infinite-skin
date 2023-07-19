-- Add up migration script here
ALTER TABLE album ADD user_id INT NOT NULL DEFAULT 1;
ALTER TABLE album ADD is_private BOOLEAN NOT NULL DEFAULT TRUE;

ALTER TABLE album ADD CONSTRAINT FK_AlbumUserId FOREIGN KEY (user_id) REFERENCES user(id);
