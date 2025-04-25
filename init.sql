CREATE DATABASE IF NOT EXISTS `file-db`;
USE `file-db`;

CREATE TABLE Files(
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(100) NOT NULL,
    file_name VARCHAR(100) NOT NULL,
    file_type VARCHAR(100) NOT NULL,
    contents LONGBLOB NOT NULL
);
