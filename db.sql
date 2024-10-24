-- Active: 1721097763988@@127.0.0.1@3306@x_admin
CREATE DATABASE x_admin;

DROP TABLE IF EXISTS `opt_log`;
CREATE TABLE `opt_log` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'id',
  `user_id` BIGINT NOT NULL COMMENT '用户id',
  `operation` VARCHAR(100) NOT NULL COMMENT '用户操作',
	`method`      VARCHAR(100) NOT NULL DEFAULT '' COMMENT '请求方法',
	`params`     VARCHAR(3000) NOT NULL DEFAULT '' COMMENT '请求参数',
	`time`     BIGINT NOT NULL COMMENT '执行时长(毫秒)',
	`ip`      VARCHAR(100) NOT NULL DEFAULT '' COMMENT 'IP地址',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`)
) COMMENT='操作日志';


DROP TABLE IF EXISTS `user`;
CREATE TABLE `user` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'id',
  `username` VARCHAR(50) NOT NULL COMMENT '账号',
  `password` VARCHAR(50) NOT NULL COMMENT '密码',
	`email`      VARCHAR(100) NOT NULL DEFAULT '' COMMENT '邮箱',
	`mobile`     VARCHAR(100) NOT NULL DEFAULT '' COMMENT '手机号',
	`status`     TINYINT(4) NOT NULL DEFAULT '1' COMMENT '状态(0:禁用,1:正常)',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`)
) COMMENT='用户';

INSERT INTO `user`(`username`, `password`)
VALUES ('admin', '21232f297a57a5a743894a0e4a801fc3');


DROP TABLE IF EXISTS `menu`;
CREATE TABLE `menu` (
	`id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'id',
	`pid` BIGINT UNSIGNED NOT NULL DEFAULT '0' COMMENT '父id，一级菜单为0',
	`name` VARCHAR ( 50 ) NOT NULL COMMENT '菜单名称',
	`path` VARCHAR ( 200 ) NOT NULL DEFAULT '' COMMENT '路由路径',
	`component` VARCHAR ( 200 ) NOT NULL DEFAULT '' COMMENT '组件路径',
	`icon` VARCHAR ( 300 ) NOT NULL DEFAULT '' COMMENT '菜单图标',
	`perms` VARCHAR ( 500 ) NOT NULL DEFAULT '' COMMENT '授权(user:add)',
	`type` INT NOT NULL COMMENT '类型(1：目录,2：菜单,3：按钮)',
	`sort` INT NOT NULL DEFAULT '100' COMMENT '排序',
	`status` INT NOT NULL DEFAULT '1' COMMENT '状态(0:禁用,1:启用,2:隐藏)',
	`created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
	`updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
	PRIMARY KEY ( `id` ) 
) COMMENT = '菜单';

INSERT INTO `menu`(`id`,`pid`,`name`,`path`,`component`,`icon`,`perms`,`type`) VALUES
(1, 0, '系统管理', '/sys', '', '', '', 1),
(2, 1, '菜单管理', 'menu', '/sys/menu', '', '', 2),
(3, 2, '新增', '', '', '', 'menu:add', 3),
(4, 2, '修改', '', '', '', 'menu:update', 3),
(5, 2, '删除', '', '', '', 'menu:delete', 3),
(6, 1, '用户管理', 'user', '/sys/user', '', '', 2),
(7, 6, '新增', '', '', '', 'user:add', 3),
(8, 6, '修改', '', '', '', 'user:update', 3),
(9, 6, '删除', '', '', '', 'user:delete', 3),
(10, 1, '角色管理', 'role', '/sys/role', '', '', 2),
(11, 10, '新增', '', '', '', 'role:add', 3),
(12, 10, '修改', '', '', '', 'role:update', 3),
(13, 10, '删除', '', '', '', 'role:delete', 3);


DROP TABLE IF EXISTS `role`;
CREATE TABLE `role` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'id',
  `name` varchar(50) NOT NULL COMMENT '名字',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`)
) COMMENT='角色';

INSERT INTO `role`(`name`) VALUES
 ('研发'),
 ('测试'),
 ('产品');


DROP TABLE IF EXISTS `role_menu`;
CREATE TABLE `role_menu` (
  `id` BIGINT unsigned NOT NULL AUTO_INCREMENT COMMENT 'id',
  `role_id` BIGINT unsigned NOT NULL COMMENT '角色ID',
  `menu_id` BIGINT unsigned NOT NULL COMMENT '菜单ID',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`)
) COMMENT='角色菜单';

INSERT INTO `role_menu`(`role_id`,`menu_id`) VALUES
(1,1),
(1,2),
(1,3),
(1,4),
(1,5),
(1,6),
(1,7),
(1,8),
(1,9),
(1,10),
(1,11),
(1,12);


DROP TABLE IF EXISTS `user_role`;
CREATE TABLE `user_role` (
  `id` BIGINT(10) UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'id',
  `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID',
  `role_id` BIGINT UNSIGNED NOT NULL COMMENT '角色ID',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`)
) COMMENT='用户角色';

INSERT INTO `user_role`(`user_id`,`role_id`) VALUES (1,1);