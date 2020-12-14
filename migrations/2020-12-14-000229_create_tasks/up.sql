-- Your SQL goes here
CREATE TABLE `todo`.`tasks` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(125) NOT NULL,
  `is_done` VARCHAR(5) NOT NULL,
  PRIMARY KEY (`id`));

INSERT INTO `todo`.`tasks`
(`name`,`is_done`)
VALUES
('test','false');

INSERT INTO `todo`.`tasks`
(`name`,`is_done`)
VALUES
('test2','false');