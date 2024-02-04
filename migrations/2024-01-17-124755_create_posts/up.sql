-- Your SQL goes here
CREATE TABLE `posts`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`date` TIMESTAMP NOT NULL,
	`title` VARCHAR NOT NULL,
	`body` TEXT NOT NULL,
	`published` BOOL NOT NULL
);

