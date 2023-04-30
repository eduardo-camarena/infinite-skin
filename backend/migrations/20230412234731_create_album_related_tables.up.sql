-- Add up migration script here
CREATE TABLE series(
  id INT AUTO_INCREMENT PRIMARY KEY,
  name TINYTEXT
);

CREATE TABLE album(
  id INT AUTO_INCREMENT PRIMARY KEY,
  name TINYTEXT NOT NULL,
  pages SMALLINT NOT NULL,
  artist_id INT,
  FOREIGN KEY (artist_id) REFERENCES artist(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE album_series(
  album_id INT AUTO_INCREMENT PRIMARY KEY,
  series_id INT,
  chapter_number TINYINT,
  FOREIGN KEY (album_id) REFERENCES album(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
  FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
  UNIQUE(album_id, series_id, chapter_number)
);

CREATE TABLE album_info(
  id INT AUTO_INCREMENT PRIMARY KEY,
  description TEXT NOT NULL,
  rating TINYINT NOT NULL CHECK (rating >= 0 AND rating <= 10),
  album_id INT NOT NULL,
  FOREIGN KEY (album_id) REFERENCES album(id) ON DELETE NO ACTION ON UPDATE NO ACTION
);
