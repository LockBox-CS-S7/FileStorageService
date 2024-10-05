CREATE DATABASE IF NOT EXISTS `file-db`;
USE `file-db`;


CREATE TABLE Files(
    FileId int AUTO_INCREMENT PRIMARY KEY,
    FileName varchar(100) not null,
    FileType varchar(100) not null,
    Contents BLOB not null
);
