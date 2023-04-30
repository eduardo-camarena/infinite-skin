-- Add up migration script here
CREATE TABLE artist(
  id INT AUTO_INCREMENT PRIMARY KEY,
  name TINYTEXT NOT NULL UNIQUE
);
