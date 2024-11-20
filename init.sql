CREATE DATABASE IF NOT EXISTS `file-db`;
USE `file-db`;


CREATE TABLE Files(
    file_id int AUTO_INCREMENT PRIMARY KEY,
    user_id varchar(100) not null,
    file_name varchar(100) not null,
    file_type varchar(100) not null,
    contents LONGBLOB not null
);
