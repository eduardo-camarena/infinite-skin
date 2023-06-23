-- Add up migration script here
CREATE TABLE user(
  id INT AUTO_INCREMENT PRIMARY KEY,
  username TINYTEXT NOT NULL UNIQUE,
  role VARCHAR(10)  NOT NULL,
  password TEXT NOT NULL
);
