CREATE SCHEMA IF NOT EXISTS `db` DEFAULT CHARACTER SET utf8mb4 ;
USE `db` ;

SET FOREIGN_KEY_CHECKS=0;

CREATE TABLE IF NOT EXISTS `users` (
  `id` VARCHAR(255) NOT NULL,
  `name` VARCHAR(255) NOT NULL,
  `created_at` DATETIME NOT NULL,
  `updated_at` DATETIME NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB DEFAULT CHARSET=utf8mb4
COMMENT = '';

SET FOREIGN_KEY_CHECKS=1;
