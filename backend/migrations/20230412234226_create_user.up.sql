-- Add up migration script here
CREATE TABLE user(
  id INT AUTO_INCREMENT PRIMARY KEY,
  username TINYTEXT NOT NULL,
  email TINYTEXT NOT NULL,
  password TEXT NOT NULL
);