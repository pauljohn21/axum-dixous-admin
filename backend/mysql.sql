-- Adminer 4.8.1 MySQL 8.0.27 dump

SET NAMES utf8;
SET time_zone = '+00:00';
SET foreign_key_checks = 0;
SET sql_mode = 'NO_AUTO_VALUE_ON_ZERO';

SET NAMES utf8mb4;

CREATE DATABASE `gva` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;
USE `gva`;

DROP TABLE IF EXISTS `casbin_rule`;
CREATE TABLE `casbin_rule` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `ptype` varchar(100) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `v0` varchar(100) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `v1` varchar(100) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `v2` varchar(100) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `v3` varchar(100) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `v4` varchar(100) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `v5` varchar(100) COLLATE utf8mb4_general_ci DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_casbin_rule` (`ptype`,`v0`,`v1`,`v2`,`v3`,`v4`,`v5`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `casbin_rule` (`id`, `ptype`, `v0`, `v1`, `v2`, `v3`, `v4`, `v5`) VALUES
(2,	'p',	'888',	'/api/createApi',	'POST',	'',	'',	''),
(5,	'p',	'888',	'/api/deleteApi',	'POST',	'',	'',	''),
(8,	'p',	'888',	'/api/deleteApisByIds',	'DELETE',	'',	'',	''),
(7,	'p',	'888',	'/api/getAllApis',	'POST',	'',	'',	''),
(4,	'p',	'888',	'/api/getApiById',	'POST',	'',	'',	''),
(3,	'p',	'888',	'/api/getApiList',	'POST',	'',	'',	''),
(6,	'p',	'888',	'/api/updateApi',	'POST',	'',	'',	''),
(9,	'p',	'888',	'/role/copyrole',	'POST',	'',	'',	''),
(11,	'p',	'888',	'/role/createrole',	'POST',	'',	'',	''),
(12,	'p',	'888',	'/role/deleterole',	'POST',	'',	'',	''),
(13,	'p',	'888',	'/role/getroleList',	'POST',	'',	'',	''),
(14,	'p',	'888',	'/role/setDatarole',	'POST',	'',	'',	''),
(10,	'p',	'888',	'/role/updaterole',	'PUT',	'',	'',	''),
(89,	'p',	'888',	'/roleBtn/canRemoveroleBtn',	'POST',	'',	'',	''),
(88,	'p',	'888',	'/roleBtn/getroleBtn',	'POST',	'',	'',	''),
(87,	'p',	'888',	'/roleBtn/setroleBtn',	'POST',	'',	'',	''),
(61,	'p',	'888',	'/autoCode/createPackage',	'POST',	'',	'',	''),
(64,	'p',	'888',	'/autoCode/createPlug',	'POST',	'',	'',	''),
(58,	'p',	'888',	'/autoCode/createTemp',	'POST',	'',	'',	''),
(63,	'p',	'888',	'/autoCode/delPackage',	'POST',	'',	'',	''),
(59,	'p',	'888',	'/autoCode/delSysHistory',	'POST',	'',	'',	''),
(56,	'p',	'888',	'/autoCode/getColumn',	'GET',	'',	'',	''),
(52,	'p',	'888',	'/autoCode/getDB',	'GET',	'',	'',	''),
(53,	'p',	'888',	'/autoCode/getMeta',	'POST',	'',	'',	''),
(62,	'p',	'888',	'/autoCode/getPackage',	'POST',	'',	'',	''),
(60,	'p',	'888',	'/autoCode/getSysHistory',	'POST',	'',	'',	''),
(55,	'p',	'888',	'/autoCode/getTables',	'GET',	'',	'',	''),
(65,	'p',	'888',	'/autoCode/installPlugin',	'POST',	'',	'',	''),
(54,	'p',	'888',	'/autoCode/preview',	'POST',	'',	'',	''),
(66,	'p',	'888',	'/autoCode/pubPlug',	'POST',	'',	'',	''),
(57,	'p',	'888',	'/autoCode/rollback',	'POST',	'',	'',	''),
(42,	'p',	'888',	'/casbin/getPolicyPathByroleId',	'POST',	'',	'',	''),
(41,	'p',	'888',	'/casbin/updateCasbin',	'POST',	'',	'',	''),
(50,	'p',	'888',	'/customer/customer',	'DELETE',	'',	'',	''),
(47,	'p',	'888',	'/customer/customer',	'GET',	'',	'',	''),
(49,	'p',	'888',	'/customer/customer',	'POST',	'',	'',	''),
(48,	'p',	'888',	'/customer/customer',	'PUT',	'',	'',	''),
(51,	'p',	'888',	'/customer/customerList',	'GET',	'',	'',	''),
(83,	'p',	'888',	'/email/emailTest',	'POST',	'',	'',	''),
(35,	'p',	'888',	'/fileUploadAndDownload/breakpointContinue',	'POST',	'',	'',	''),
(34,	'p',	'888',	'/fileUploadAndDownload/breakpointContinueFinish',	'POST',	'',	'',	''),
(38,	'p',	'888',	'/fileUploadAndDownload/deleteFile',	'POST',	'',	'',	''),
(39,	'p',	'888',	'/fileUploadAndDownload/editFileName',	'POST',	'',	'',	''),
(33,	'p',	'888',	'/fileUploadAndDownload/findFile',	'GET',	'',	'',	''),
(40,	'p',	'888',	'/fileUploadAndDownload/getFileList',	'POST',	'',	'',	''),
(36,	'p',	'888',	'/fileUploadAndDownload/removeChunk',	'POST',	'',	'',	''),
(37,	'p',	'888',	'/fileUploadAndDownload/upload',	'POST',	'',	'',	''),
(43,	'p',	'888',	'/jwt/jsonInBlacklist',	'POST',	'',	'',	''),
(17,	'p',	'888',	'/menu/addBaseMenu',	'POST',	'',	'',	''),
(19,	'p',	'888',	'/menu/addMenurole',	'POST',	'',	'',	''),
(21,	'p',	'888',	'/menu/deleteBaseMenu',	'POST',	'',	'',	''),
(23,	'p',	'888',	'/menu/getBaseMenuById',	'POST',	'',	'',	''),
(18,	'p',	'888',	'/menu/getBaseMenuTree',	'POST',	'',	'',	''),
(15,	'p',	'888',	'/menu/getMenu',	'POST',	'',	'',	''),
(20,	'p',	'888',	'/menu/getMenurole',	'POST',	'',	'',	''),
(16,	'p',	'888',	'/menu/getMenuList',	'POST',	'',	'',	''),
(22,	'p',	'888',	'/menu/updateBaseMenu',	'POST',	'',	'',	''),
(85,	'p',	'888',	'/simpleUploader/checkFileMd5',	'GET',	'',	'',	''),
(86,	'p',	'888',	'/simpleUploader/mergeFileMd5',	'GET',	'',	'',	''),
(84,	'p',	'888',	'/simpleUploader/upload',	'POST',	'',	'',	''),
(75,	'p',	'888',	'/sysDictionary/createSysDictionary',	'POST',	'',	'',	''),
(76,	'p',	'888',	'/sysDictionary/deleteSysDictionary',	'DELETE',	'',	'',	''),
(72,	'p',	'888',	'/sysDictionary/findSysDictionary',	'GET',	'',	'',	''),
(74,	'p',	'888',	'/sysDictionary/getSysDictionaryList',	'GET',	'',	'',	''),
(73,	'p',	'888',	'/sysDictionary/updateSysDictionary',	'PUT',	'',	'',	''),
(69,	'p',	'888',	'/sysDictionaryDetail/createSysDictionaryDetail',	'POST',	'',	'',	''),
(71,	'p',	'888',	'/sysDictionaryDetail/deleteSysDictionaryDetail',	'DELETE',	'',	'',	''),
(67,	'p',	'888',	'/sysDictionaryDetail/findSysDictionaryDetail',	'GET',	'',	'',	''),
(70,	'p',	'888',	'/sysDictionaryDetail/getSysDictionaryDetailList',	'GET',	'',	'',	''),
(68,	'p',	'888',	'/sysDictionaryDetail/updateSysDictionaryDetail',	'PUT',	'',	'',	''),
(90,	'p',	'888',	'/sysExportTemplate/createSysExportTemplate',	'POST',	'',	'',	''),
(91,	'p',	'888',	'/sysExportTemplate/deleteSysExportTemplate',	'DELETE',	'',	'',	''),
(92,	'p',	'888',	'/sysExportTemplate/deleteSysExportTemplateByIds',	'DELETE',	'',	'',	''),
(96,	'p',	'888',	'/sysExportTemplate/exportExcel',	'GET',	'',	'',	''),
(97,	'p',	'888',	'/sysExportTemplate/exportTemplate',	'GET',	'',	'',	''),
(94,	'p',	'888',	'/sysExportTemplate/findSysExportTemplate',	'GET',	'',	'',	''),
(95,	'p',	'888',	'/sysExportTemplate/getSysExportTemplateList',	'GET',	'',	'',	''),
(98,	'p',	'888',	'/sysExportTemplate/importExcel',	'POST',	'',	'',	''),
(93,	'p',	'888',	'/sysExportTemplate/updateSysExportTemplate',	'PUT',	'',	'',	''),
(79,	'p',	'888',	'/sysOperationRecord/createSysOperationRecord',	'POST',	'',	'',	''),
(81,	'p',	'888',	'/sysOperationRecord/deleteSysOperationRecord',	'DELETE',	'',	'',	''),
(82,	'p',	'888',	'/sysOperationRecord/deleteSysOperationRecordByIds',	'DELETE',	'',	'',	''),
(77,	'p',	'888',	'/sysOperationRecord/findSysOperationRecord',	'GET',	'',	'',	''),
(80,	'p',	'888',	'/sysOperationRecord/getSysOperationRecordList',	'GET',	'',	'',	''),
(78,	'p',	'888',	'/sysOperationRecord/updateSysOperationRecord',	'PUT',	'',	'',	''),
(46,	'p',	'888',	'/system/getServerInfo',	'POST',	'',	'',	''),
(44,	'p',	'888',	'/system/getSystemConfig',	'POST',	'',	'',	''),
(45,	'p',	'888',	'/system/setSystemConfig',	'POST',	'',	'',	''),
(1,	'p',	'888',	'/user/admin_register',	'POST',	'',	'',	''),
(29,	'p',	'888',	'/user/changePassword',	'POST',	'',	'',	''),
(28,	'p',	'888',	'/user/deleteUser',	'DELETE',	'',	'',	''),
(24,	'p',	'888',	'/user/getUserInfo',	'GET',	'',	'',	''),
(27,	'p',	'888',	'/user/getUserList',	'POST',	'',	'',	''),
(32,	'p',	'888',	'/user/resetPassword',	'POST',	'',	'',	''),
(26,	'p',	'888',	'/user/setSelfInfo',	'PUT',	'',	'',	''),
(31,	'p',	'888',	'/user/setUserrole',	'POST',	'',	'',	''),
(30,	'p',	'888',	'/user/setUserrole',	'POST',	'',	'',	''),
(25,	'p',	'888',	'/user/setUserInfo',	'PUT',	'',	'',	''),
(100,	'p',	'8881',	'/api/createApi',	'POST',	'',	'',	''),
(103,	'p',	'8881',	'/api/deleteApi',	'POST',	'',	'',	''),
(105,	'p',	'8881',	'/api/getAllApis',	'POST',	'',	'',	''),
(102,	'p',	'8881',	'/api/getApiById',	'POST',	'',	'',	''),
(101,	'p',	'8881',	'/api/getApiList',	'POST',	'',	'',	''),
(104,	'p',	'8881',	'/api/updateApi',	'POST',	'',	'',	''),
(106,	'p',	'8881',	'/role/createrole',	'POST',	'',	'',	''),
(107,	'p',	'8881',	'/role/deleterole',	'POST',	'',	'',	''),
(108,	'p',	'8881',	'/role/getroleList',	'POST',	'',	'',	''),
(109,	'p',	'8881',	'/role/setDatarole',	'POST',	'',	'',	''),
(127,	'p',	'8881',	'/casbin/getPolicyPathByroleId',	'POST',	'',	'',	''),
(126,	'p',	'8881',	'/casbin/updateCasbin',	'POST',	'',	'',	''),
(133,	'p',	'8881',	'/customer/customer',	'DELETE',	'',	'',	''),
(134,	'p',	'8881',	'/customer/customer',	'GET',	'',	'',	''),
(131,	'p',	'8881',	'/customer/customer',	'POST',	'',	'',	''),
(132,	'p',	'8881',	'/customer/customer',	'PUT',	'',	'',	''),
(135,	'p',	'8881',	'/customer/customerList',	'GET',	'',	'',	''),
(124,	'p',	'8881',	'/fileUploadAndDownload/deleteFile',	'POST',	'',	'',	''),
(125,	'p',	'8881',	'/fileUploadAndDownload/editFileName',	'POST',	'',	'',	''),
(123,	'p',	'8881',	'/fileUploadAndDownload/getFileList',	'POST',	'',	'',	''),
(122,	'p',	'8881',	'/fileUploadAndDownload/upload',	'POST',	'',	'',	''),
(128,	'p',	'8881',	'/jwt/jsonInBlacklist',	'POST',	'',	'',	''),
(112,	'p',	'8881',	'/menu/addBaseMenu',	'POST',	'',	'',	''),
(114,	'p',	'8881',	'/menu/addMenurole',	'POST',	'',	'',	''),
(116,	'p',	'8881',	'/menu/deleteBaseMenu',	'POST',	'',	'',	''),
(118,	'p',	'8881',	'/menu/getBaseMenuById',	'POST',	'',	'',	''),
(113,	'p',	'8881',	'/menu/getBaseMenuTree',	'POST',	'',	'',	''),
(110,	'p',	'8881',	'/menu/getMenu',	'POST',	'',	'',	''),
(115,	'p',	'8881',	'/menu/getMenurole',	'POST',	'',	'',	''),
(111,	'p',	'8881',	'/menu/getMenuList',	'POST',	'',	'',	''),
(117,	'p',	'8881',	'/menu/updateBaseMenu',	'POST',	'',	'',	''),
(129,	'p',	'8881',	'/system/getSystemConfig',	'POST',	'',	'',	''),
(130,	'p',	'8881',	'/system/setSystemConfig',	'POST',	'',	'',	''),
(99,	'p',	'8881',	'/user/admin_register',	'POST',	'',	'',	''),
(119,	'p',	'8881',	'/user/changePassword',	'POST',	'',	'',	''),
(136,	'p',	'8881',	'/user/getUserInfo',	'GET',	'',	'',	''),
(120,	'p',	'8881',	'/user/getUserList',	'POST',	'',	'',	''),
(121,	'p',	'8881',	'/user/setUserrole',	'POST',	'',	'',	''),
(138,	'p',	'9528',	'/api/createApi',	'POST',	'',	'',	''),
(141,	'p',	'9528',	'/api/deleteApi',	'POST',	'',	'',	''),
(143,	'p',	'9528',	'/api/getAllApis',	'POST',	'',	'',	''),
(140,	'p',	'9528',	'/api/getApiById',	'POST',	'',	'',	''),
(139,	'p',	'9528',	'/api/getApiList',	'POST',	'',	'',	''),
(142,	'p',	'9528',	'/api/updateApi',	'POST',	'',	'',	''),
(144,	'p',	'9528',	'/role/createrole',	'POST',	'',	'',	''),
(145,	'p',	'9528',	'/role/deleterole',	'POST',	'',	'',	''),
(146,	'p',	'9528',	'/role/getroleList',	'POST',	'',	'',	''),
(147,	'p',	'9528',	'/role/setDatarole',	'POST',	'',	'',	''),
(174,	'p',	'9528',	'/autoCode/createTemp',	'POST',	'',	'',	''),
(165,	'p',	'9528',	'/casbin/getPolicyPathByroleId',	'POST',	'',	'',	''),
(164,	'p',	'9528',	'/casbin/updateCasbin',	'POST',	'',	'',	''),
(172,	'p',	'9528',	'/customer/customer',	'DELETE',	'',	'',	''),
(170,	'p',	'9528',	'/customer/customer',	'GET',	'',	'',	''),
(171,	'p',	'9528',	'/customer/customer',	'POST',	'',	'',	''),
(169,	'p',	'9528',	'/customer/customer',	'PUT',	'',	'',	''),
(173,	'p',	'9528',	'/customer/customerList',	'GET',	'',	'',	''),
(162,	'p',	'9528',	'/fileUploadAndDownload/deleteFile',	'POST',	'',	'',	''),
(163,	'p',	'9528',	'/fileUploadAndDownload/editFileName',	'POST',	'',	'',	''),
(161,	'p',	'9528',	'/fileUploadAndDownload/getFileList',	'POST',	'',	'',	''),
(160,	'p',	'9528',	'/fileUploadAndDownload/upload',	'POST',	'',	'',	''),
(166,	'p',	'9528',	'/jwt/jsonInBlacklist',	'POST',	'',	'',	''),
(150,	'p',	'9528',	'/menu/addBaseMenu',	'POST',	'',	'',	''),
(152,	'p',	'9528',	'/menu/addMenurole',	'POST',	'',	'',	''),
(154,	'p',	'9528',	'/menu/deleteBaseMenu',	'POST',	'',	'',	''),
(156,	'p',	'9528',	'/menu/getBaseMenuById',	'POST',	'',	'',	''),
(151,	'p',	'9528',	'/menu/getBaseMenuTree',	'POST',	'',	'',	''),
(148,	'p',	'9528',	'/menu/getMenu',	'POST',	'',	'',	''),
(153,	'p',	'9528',	'/menu/getMenurole',	'POST',	'',	'',	''),
(149,	'p',	'9528',	'/menu/getMenuList',	'POST',	'',	'',	''),
(155,	'p',	'9528',	'/menu/updateBaseMenu',	'POST',	'',	'',	''),
(167,	'p',	'9528',	'/system/getSystemConfig',	'POST',	'',	'',	''),
(168,	'p',	'9528',	'/system/setSystemConfig',	'POST',	'',	'',	''),
(137,	'p',	'9528',	'/user/admin_register',	'POST',	'',	'',	''),
(157,	'p',	'9528',	'/user/changePassword',	'POST',	'',	'',	''),
(175,	'p',	'9528',	'/user/getUserInfo',	'GET',	'',	'',	''),
(158,	'p',	'9528',	'/user/getUserList',	'POST',	'',	'',	''),
(159,	'p',	'9528',	'/user/setUserrole',	'POST',	'',	'',	'');



DROP TABLE IF EXISTS `exa_file_chunks`;
CREATE TABLE `exa_file_chunks` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `exa_file_id` bigint unsigned DEFAULT NULL,
  `file_chunk_number` bigint DEFAULT NULL,
  `file_chunk_path` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_exa_file_chunks_deleted_at` (`deleted_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

DROP TABLE IF EXISTS `jwt_blacklists`;
CREATE TABLE `jwt_blacklists` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `jwt` text COLLATE utf8mb4_general_ci COMMENT 'jwt',
  PRIMARY KEY (`id`),
  KEY `idx_jwt_blacklists_deleted_at` (`deleted_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


DROP TABLE IF EXISTS `sys_apis`;
CREATE TABLE `sys_apis` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `path` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'api路径',
  `description` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'api中文描述',
  `api_group` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'api组',
  `method` varchar(191) COLLATE utf8mb4_general_ci DEFAULT 'POST' COMMENT '方法',
  PRIMARY KEY (`id`),
  KEY `idx_sys_apis_deleted_at` (`deleted_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_apis` (`id`, `created_at`, `updated_at`, `deleted_at`, `path`, `description`, `api_group`, `method`) VALUES
(1,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/jwt/jsonInBlacklist',	'jwt加入黑名单(退出，必选)',	'jwt',	'POST'),
(2,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/deleteUser',	'删除用户',	'系统用户',	'DELETE'),
(3,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/admin_register',	'用户注册',	'系统用户',	'POST'),
(4,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/getUserList',	'获取用户列表',	'系统用户',	'POST'),
(5,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/setUserInfo',	'设置用户信息',	'系统用户',	'PUT'),
(6,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/setSelfInfo',	'设置自身信息(必选)',	'系统用户',	'PUT'),
(7,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/getUserInfo',	'获取自身信息(必选)',	'系统用户',	'GET'),
(8,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/setUserrole',	'设置权限组',	'系统用户',	'POST'),
(9,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/changePassword',	'修改密码（建议选择)',	'系统用户',	'POST'),
(10,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/setUserrole',	'修改用户角色(必选)',	'系统用户',	'POST'),
(11,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/user/resetPassword',	'重置用户密码',	'系统用户',	'POST'),
(12,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/api/createApi',	'创建api',	'api',	'POST'),
(13,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/api/deleteApi',	'删除Api',	'api',	'POST'),
(14,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/api/updateApi',	'更新Api',	'api',	'POST'),
(15,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/api/getApiList',	'获取api列表',	'api',	'POST'),
(16,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/api/getAllApis',	'获取所有api',	'api',	'POST'),
(17,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/api/getApiById',	'获取api详细信息',	'api',	'POST'),
(18,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/api/deleteApisByIds',	'批量删除api',	'api',	'DELETE'),
(19,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/role/copyrole',	'拷贝角色',	'角色',	'POST'),
(20,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/role/createrole',	'创建角色',	'角色',	'POST'),
(21,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/role/deleterole',	'删除角色',	'角色',	'POST'),
(22,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/role/updaterole',	'更新角色信息',	'角色',	'PUT'),
(23,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/role/getroleList',	'获取角色列表',	'角色',	'POST'),
(24,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/role/setDatarole',	'设置角色资源权限',	'角色',	'POST'),
(25,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/casbin/updateCasbin',	'更改角色api权限',	'casbin',	'POST'),
(26,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/casbin/getPolicyPathByroleId',	'获取权限列表',	'casbin',	'POST'),
(27,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/addBaseMenu',	'新增菜单',	'菜单',	'POST'),
(28,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/getMenu',	'获取菜单树(必选)',	'菜单',	'POST'),
(29,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/deleteBaseMenu',	'删除菜单',	'菜单',	'POST'),
(30,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/updateBaseMenu',	'更新菜单',	'菜单',	'POST'),
(31,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/getBaseMenuById',	'根据id获取菜单',	'菜单',	'POST'),
(32,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/getMenuList',	'分页获取基础menu列表',	'菜单',	'POST'),
(33,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/getBaseMenuTree',	'获取用户动态路由',	'菜单',	'POST'),
(34,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/getMenurole',	'获取指定角色menu',	'菜单',	'POST'),
(35,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/menu/addMenurole',	'增加menu和角色关联关系',	'菜单',	'POST'),
(36,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/fileUploadAndDownload/findFile',	'寻找目标文件（秒传）',	'分片上传',	'GET'),
(37,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/fileUploadAndDownload/breakpointContinue',	'断点续传',	'分片上传',	'POST'),
(38,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/fileUploadAndDownload/breakpointContinueFinish',	'断点续传完成',	'分片上传',	'POST'),
(39,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/fileUploadAndDownload/removeChunk',	'上传完成移除文件',	'分片上传',	'POST'),
(40,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/fileUploadAndDownload/upload',	'文件上传示例',	'文件上传与下载',	'POST'),
(41,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/fileUploadAndDownload/deleteFile',	'删除文件',	'文件上传与下载',	'POST'),
(42,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/fileUploadAndDownload/editFileName',	'文件名或者备注编辑',	'文件上传与下载',	'POST'),
(43,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/fileUploadAndDownload/getFileList',	'获取上传文件列表',	'文件上传与下载',	'POST'),
(44,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/system/getServerInfo',	'获取服务器信息',	'系统服务',	'POST'),
(45,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/system/getSystemConfig',	'获取配置文件内容',	'系统服务',	'POST'),
(46,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/system/setSystemConfig',	'设置配置文件内容',	'系统服务',	'POST'),
(47,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/customer/customer',	'更新客户',	'客户',	'PUT'),
(48,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/customer/customer',	'创建客户',	'客户',	'POST'),
(49,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/customer/customer',	'删除客户',	'客户',	'DELETE'),
(50,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/customer/customer',	'获取单一客户',	'客户',	'GET'),
(51,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/customer/customerList',	'获取客户列表',	'客户',	'GET'),
(52,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/getDB',	'获取所有数据库',	'代码生成器',	'GET'),
(53,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/getTables',	'获取数据库表',	'代码生成器',	'GET'),
(54,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/createTemp',	'自动化代码',	'代码生成器',	'POST'),
(55,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/preview',	'预览自动化代码',	'代码生成器',	'POST'),
(56,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/getColumn',	'获取所选table的所有字段',	'代码生成器',	'GET'),
(57,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/createPlug',	'自动创建插件包',	'代码生成器',	'POST'),
(58,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/installPlugin',	'安装插件',	'代码生成器',	'POST'),
(59,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/pubPlug',	'打包插件',	'代码生成器',	'POST'),
(60,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/createPackage',	'生成包(package)',	'包（pkg）生成器',	'POST'),
(61,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/getPackage',	'获取所有包(package)',	'包（pkg）生成器',	'POST'),
(62,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/delPackage',	'删除包(package)',	'包（pkg）生成器',	'POST'),
(63,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/getMeta',	'获取meta信息',	'代码生成器历史',	'POST'),
(64,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/rollback',	'回滚自动生成代码',	'代码生成器历史',	'POST'),
(65,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/getSysHistory',	'查询回滚记录',	'代码生成器历史',	'POST'),
(66,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/autoCode/delSysHistory',	'删除回滚记录',	'代码生成器历史',	'POST'),
(67,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionaryDetail/updateSysDictionaryDetail',	'更新字典内容',	'系统字典详情',	'PUT'),
(68,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionaryDetail/createSysDictionaryDetail',	'新增字典内容',	'系统字典详情',	'POST'),
(69,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionaryDetail/deleteSysDictionaryDetail',	'删除字典内容',	'系统字典详情',	'DELETE'),
(70,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionaryDetail/findSysDictionaryDetail',	'根据ID获取字典内容',	'系统字典详情',	'GET'),
(71,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionaryDetail/getSysDictionaryDetailList',	'获取字典内容列表',	'系统字典详情',	'GET'),
(72,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionary/createSysDictionary',	'新增字典',	'系统字典',	'POST'),
(73,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionary/deleteSysDictionary',	'删除字典',	'系统字典',	'DELETE'),
(74,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionary/updateSysDictionary',	'更新字典',	'系统字典',	'PUT'),
(75,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionary/findSysDictionary',	'根据ID获取字典',	'系统字典',	'GET'),
(76,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysDictionary/getSysDictionaryList',	'获取字典列表',	'系统字典',	'GET'),
(77,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysOperationRecord/createSysOperationRecord',	'新增操作记录',	'操作记录',	'POST'),
(78,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysOperationRecord/findSysOperationRecord',	'根据ID获取操作记录',	'操作记录',	'GET'),
(79,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysOperationRecord/getSysOperationRecordList',	'获取操作记录列表',	'操作记录',	'GET'),
(80,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysOperationRecord/deleteSysOperationRecord',	'删除操作记录',	'操作记录',	'DELETE'),
(81,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysOperationRecord/deleteSysOperationRecordByIds',	'批量删除操作历史',	'操作记录',	'DELETE'),
(82,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/simpleUploader/upload',	'插件版分片上传',	'断点续传(插件版)',	'POST'),
(83,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/simpleUploader/checkFileMd5',	'文件完整度验证',	'断点续传(插件版)',	'GET'),
(84,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/simpleUploader/mergeFileMd5',	'上传完成合并文件',	'断点续传(插件版)',	'GET'),
(85,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/email/emailTest',	'发送测试邮件',	'email',	'POST'),
(86,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/email/emailSend',	'发送邮件示例',	'email',	'POST'),
(87,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/roleBtn/setroleBtn',	'设置按钮权限',	'按钮权限',	'POST'),
(88,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/roleBtn/getroleBtn',	'获取已有按钮权限',	'按钮权限',	'POST'),
(89,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/roleBtn/canRemoveroleBtn',	'删除按钮',	'按钮权限',	'POST'),
(90,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/createSysExportTemplate',	'新增导出模板',	'表格模板',	'POST'),
(91,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/deleteSysExportTemplate',	'删除导出模板',	'表格模板',	'DELETE'),
(92,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/deleteSysExportTemplateByIds',	'批量删除导出模板',	'表格模板',	'DELETE'),
(93,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/updateSysExportTemplate',	'更新导出模板',	'表格模板',	'PUT'),
(94,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/findSysExportTemplate',	'根据ID获取导出模板',	'表格模板',	'GET'),
(95,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/getSysExportTemplateList',	'获取导出模板列表',	'表格模板',	'GET'),
(96,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/exportExcel',	'导出Excel',	'表格模板',	'GET'),
(97,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/exportTemplate',	'下载模板',	'表格模板',	'GET'),
(98,	'2024-04-23 11:20:33.270',	'2024-04-23 11:20:33.270',	NULL,	'/sysExportTemplate/importExcel',	'导入Excel',	'表格模板',	'POST');

DROP TABLE IF EXISTS `sys_role`;
CREATE TABLE `sys_role` (
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `role_id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '角色ID',
  `role_name` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '角色名',
  `parent_id` bigint unsigned DEFAULT NULL COMMENT '父角色ID',
  `default_router` varchar(191) COLLATE utf8mb4_general_ci DEFAULT 'dashboard' COMMENT '默认菜单',
  PRIMARY KEY (`role_id`),
  UNIQUE KEY `uni_sys_role_role_id` (`role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_role` (`created_at`, `updated_at`, `deleted_at`, `role_id`, `role_name`, `parent_id`, `default_router`) VALUES
('2024-04-23 11:20:33.284',	'2024-04-23 11:20:33.479',	NULL,	888,	'普通用户',	0,	'dashboard'),
('2024-04-23 11:20:33.284',	'2024-04-23 11:20:33.502',	NULL,	8881,	'普通用户子角色',	888,	'dashboard'),
('2024-04-23 11:20:33.284',	'2024-04-23 11:20:33.490',	NULL,	9528,	'测试角色',	0,	'dashboard');


DROP TABLE IF EXISTS `sys_role_menus`;
CREATE TABLE `sys_role_menus` (
  `sys_base_menu_id` bigint unsigned NOT NULL,
  `sys_role_role_id` bigint unsigned NOT NULL COMMENT '角色ID',
  PRIMARY KEY (`sys_base_menu_id`,`sys_role_role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_role_menus` (`sys_base_menu_id`, `sys_role_role_id`) VALUES
(1,	888),
(1,	8881),
(1,	9528),
(2,	888),
(2,	8881),
(2,	9528),
(3,	888),
(3,	8881),
(4,	888),
(4,	8881),
(5,	888),
(5,	8881),
(6,	888),
(6,	8881),
(7,	888),
(7,	8881),
(7,	9528),
(8,	888),
(8,	8881),
(9,	888),
(9,	8881);





DROP TABLE IF EXISTS `sys_base_menus`;
CREATE TABLE `sys_base_menus` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `menu_level` bigint unsigned DEFAULT NULL,
  `parent_id` bigint unsigned DEFAULT NULL COMMENT '父菜单ID',
  `path` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '路由path',
  `name` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '路由name',
  `hidden` tinyint(1) DEFAULT NULL COMMENT '是否在列表隐藏',
  `component` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '对应前端文件路径',
  `sort` bigint DEFAULT NULL COMMENT '排序标记',
  `active_name` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '附加属性',
  `keep_alive` tinyint(1) DEFAULT NULL COMMENT '附加属性',
  `default_menu` tinyint(1) DEFAULT NULL COMMENT '附加属性',
  `title` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '附加属性',
  `icon` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '附加属性',
  `close_tab` tinyint(1) DEFAULT NULL COMMENT '附加属性',
  PRIMARY KEY (`id`),
  KEY `idx_sys_base_menus_deleted_at` (`deleted_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_base_menus` (`id`, `created_at`, `updated_at`, `deleted_at`, `menu_level`, `parent_id`, `path`, `name`, `hidden`, `component`, `sort`, `active_name`, `keep_alive`, `default_menu`, `title`, `icon`, `close_tab`) VALUES
(1,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	0,	'dashboard',	'dashboard',	0,	'',	1,	'',	0,	0,	'仪表盘',	'odometer',	0),
(2,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	0,	'admin',	'superAdmin',	0,	'',	2,	'',	0,	0,	'超级管理员',	'user',	0),
(3,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	2,	'role',	'role',	0,	'',	1,	'',	0,	0,	'角色管理',	'avatar',	0),
(4,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	2,	'menu',	'menu',	0,	'',	2,	'',	0,	0,	'菜单管理',	'tickets',	0),
(5,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	2,	'api',	'api',	0,	'',	3,	'',	0,	0,	'API管理',	'platform',	0),
(6,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	2,	'user',	'user',	0,	'',	4,	'',	0,	0,	'用户管理',	'coordinate',	0),
(7,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	0,	'dictionary',	'dictionary',	0,	'',	3,	'',	0,	0,	'字典管理',	'dict',	0),
(8,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	0,	'profile',	'profile',	0,	'',	4,	'',	0,	0,	'个人信息',	'message',	0),
(9,	'2024-04-23 11:20:33.340',	'2024-04-23 11:20:33.340',	NULL,	0,	0,	'settings',	'settings',	0,	'',	5,	'',	0,	0,	'系统设置',	'setting',	0);

DROP TABLE IF EXISTS `sys_data_role_id`;
CREATE TABLE `sys_data_role_id` (
  `sys_role_role_id` bigint unsigned NOT NULL COMMENT '角色ID',
  `data_role_id_role_id` bigint unsigned NOT NULL COMMENT '角色ID',
  PRIMARY KEY (`sys_role_role_id`,`data_role_id_role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_data_role_id` (`sys_role_role_id`, `data_role_id_role_id`) VALUES
(888,	888),
(888,	8881),
(888,	9528),
(9528,	8881),
(9528,	9528);

DROP TABLE IF EXISTS `sys_dictionaries`;
CREATE TABLE `sys_dictionaries` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `name` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '字典名（中）',
  `type` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '字典名（英）',
  `status` tinyint(1) DEFAULT NULL COMMENT '状态',
  `desc` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '描述',
  PRIMARY KEY (`id`),
  KEY `idx_sys_dictionaries_deleted_at` (`deleted_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_dictionaries` (`id`, `created_at`, `updated_at`, `deleted_at`, `name`, `type`, `status`, `desc`) VALUES
(1,	'2024-04-23 11:20:33.300',	'2024-04-23 11:20:33.305',	NULL,	'性别',	'gender',	1,	'性别字典'),
(2,	'2024-04-23 11:20:33.300',	'2024-04-23 11:20:33.311',	NULL,	'数据库int类型',	'int',	1,	'int类型对应的数据库类型'),
(3,	'2024-04-23 11:20:33.300',	'2024-04-23 11:20:33.317',	NULL,	'数据库时间日期类型',	'time.Time',	1,	'数据库时间日期类型'),
(4,	'2024-04-23 11:20:33.300',	'2024-04-23 11:20:33.322',	NULL,	'数据库浮点型',	'float64',	1,	'数据库浮点型'),
(5,	'2024-04-23 11:20:33.300',	'2024-04-23 11:20:33.328',	NULL,	'数据库字符串',	'string',	1,	'数据库字符串'),
(6,	'2024-04-23 11:20:33.300',	'2024-04-23 11:20:33.334',	NULL,	'数据库bool类型',	'bool',	1,	'数据库bool类型');

DROP TABLE IF EXISTS `sys_dictionary_details`;
CREATE TABLE `sys_dictionary_details` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `label` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '展示值',
  `value` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '字典值',
  `extend` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '扩展值',
  `status` tinyint(1) DEFAULT NULL COMMENT '启用状态',
  `sort` bigint DEFAULT NULL COMMENT '排序标记',
  `sys_dictionary_id` bigint unsigned DEFAULT NULL COMMENT '关联标记',
  PRIMARY KEY (`id`),
  KEY `idx_sys_dictionary_details_deleted_at` (`deleted_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_dictionary_details` (`id`, `created_at`, `updated_at`, `deleted_at`, `label`, `value`, `extend`, `status`, `sort`, `sys_dictionary_id`) VALUES
(1,	'2024-04-23 11:20:33.306',	'2024-04-23 11:20:33.306',	NULL,	'男',	'1',	'',	1,	1,	1),
(2,	'2024-04-23 11:20:33.306',	'2024-04-23 11:20:33.306',	NULL,	'女',	'2',	'',	1,	2,	1),
(3,	'2024-04-23 11:20:33.312',	'2024-04-23 11:20:33.312',	NULL,	'smallint',	'1',	'mysql',	1,	1,	2),
(4,	'2024-04-23 11:20:33.312',	'2024-04-23 11:20:33.312',	NULL,	'mediumint',	'2',	'mysql',	1,	2,	2),
(5,	'2024-04-23 11:20:33.312',	'2024-04-23 11:20:33.312',	NULL,	'int',	'3',	'mysql',	1,	3,	2),
(6,	'2024-04-23 11:20:33.312',	'2024-04-23 11:20:33.312',	NULL,	'bigint',	'4',	'mysql',	1,	4,	2),
(7,	'2024-04-23 11:20:33.312',	'2024-04-23 11:20:33.312',	NULL,	'int2',	'5',	'pgsql',	1,	5,	2),
(8,	'2024-04-23 11:20:33.312',	'2024-04-23 11:20:33.312',	NULL,	'int4',	'6',	'pgsql',	1,	6,	2),
(9,	'2024-04-23 11:20:33.312',	'2024-04-23 11:20:33.312',	NULL,	'int6',	'7',	'pgsql',	1,	7,	2),
(10,	'2024-04-23 11:20:33.312',	'2024-04-23 11:20:33.312',	NULL,	'int8',	'8',	'pgsql',	1,	8,	2),
(11,	'2024-04-23 11:20:33.317',	'2024-04-23 11:20:33.317',	NULL,	'date',	'',	'',	1,	0,	3),
(12,	'2024-04-23 11:20:33.317',	'2024-04-23 11:20:33.317',	NULL,	'time',	'1',	'mysql',	1,	1,	3),
(13,	'2024-04-23 11:20:33.317',	'2024-04-23 11:20:33.317',	NULL,	'year',	'2',	'mysql',	1,	2,	3),
(14,	'2024-04-23 11:20:33.317',	'2024-04-23 11:20:33.317',	NULL,	'datetime',	'3',	'mysql',	1,	3,	3),
(15,	'2024-04-23 11:20:33.317',	'2024-04-23 11:20:33.317',	NULL,	'timestamp',	'5',	'mysql',	1,	5,	3),
(16,	'2024-04-23 11:20:33.317',	'2024-04-23 11:20:33.317',	NULL,	'timestamptz',	'6',	'pgsql',	1,	5,	3),
(17,	'2024-04-23 11:20:33.323',	'2024-04-23 11:20:33.323',	NULL,	'float',	'',	'',	1,	0,	4),
(18,	'2024-04-23 11:20:33.323',	'2024-04-23 11:20:33.323',	NULL,	'double',	'1',	'mysql',	1,	1,	4),
(19,	'2024-04-23 11:20:33.323',	'2024-04-23 11:20:33.323',	NULL,	'decimal',	'2',	'mysql',	1,	2,	4),
(20,	'2024-04-23 11:20:33.323',	'2024-04-23 11:20:33.323',	NULL,	'numeric',	'3',	'pgsql',	1,	3,	4),
(21,	'2024-04-23 11:20:33.323',	'2024-04-23 11:20:33.323',	NULL,	'smallserial',	'4',	'pgsql',	1,	4,	4),
(22,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'char',	'',	'',	1,	0,	5),
(23,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'varchar',	'1',	'mysql',	1,	1,	5),
(24,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'tinyblob',	'2',	'mysql',	1,	2,	5),
(25,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'tinytext',	'3',	'mysql',	1,	3,	5),
(26,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'text',	'4',	'mysql',	1,	4,	5),
(27,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'blob',	'5',	'mysql',	1,	5,	5),
(28,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'mediumblob',	'6',	'mysql',	1,	6,	5),
(29,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'mediumtext',	'7',	'mysql',	1,	7,	5),
(30,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'longblob',	'8',	'mysql',	1,	8,	5),
(31,	'2024-04-23 11:20:33.329',	'2024-04-23 11:20:33.329',	NULL,	'longtext',	'9',	'mysql',	1,	9,	5),
(32,	'2024-04-23 11:20:33.334',	'2024-04-23 11:20:33.334',	NULL,	'tinyint',	'1',	'mysql',	1,	0,	6),
(33,	'2024-04-23 11:20:33.334',	'2024-04-23 11:20:33.334',	NULL,	'bool',	'2',	'pgsql',	1,	0,	6);




DROP TABLE IF EXISTS `sys_operation_records`;
CREATE TABLE `sys_operation_records` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `ip` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '请求ip',
  `method` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '请求方法',
  `path` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '请求路径',
  `status` bigint DEFAULT NULL COMMENT '请求状态',
  `latency` bigint DEFAULT NULL COMMENT '延迟',
  `agent` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '代理',
  `error_message` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '错误信息',
  `body` text COLLATE utf8mb4_general_ci COMMENT '请求Body',
  `resp` text COLLATE utf8mb4_general_ci COMMENT '响应Body',
  `user_id` bigint unsigned DEFAULT NULL COMMENT '用户id',
  PRIMARY KEY (`id`),
  KEY `idx_sys_operation_records_deleted_at` (`deleted_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_operation_records` (`id`, `created_at`, `updated_at`, `deleted_at`, `ip`, `method`, `path`, `status`, `latency`, `agent`, `error_message`, `body`, `resp`, `user_id`) VALUES
(1,	'2024-04-23 11:21:20.133',	'2024-04-23 11:21:20.133',	NULL,	'127.0.0.1',	'POST',	'/system/getServerInfo',	200,	201249638,	'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0',	'',	'',	'{\"code\":0,\"data\":{\"server\":{\"os\":{\"goos\":\"darwin\",\"numCpu\":16,\"compiler\":\"gc\",\"goVersion\":\"go1.22.0\",\"numGoroutine\":11},\"cpu\":{\"cpus\":[20.000000000136424,4.5474735088480976e-10,20,0,14.285714285590545,0,5.263157894862811,0,5.000000000113687,0,9.523809523582665,0,0,0,0,0],\"cores\":8},\"ram\":{\"usedMb\":16556,\"totalMb\":32768,\"usedPercent\":50},\"disk\":{\"usedMb\":82144,\"usedGb\":80,\"totalMb\":953904,\"totalGb\":931,\"usedPercent\":8}}},\"msg\":\"获取成功\"}',	1),
(2,	'2024-04-23 11:21:30.906',	'2024-04-23 11:21:30.906',	NULL,	'127.0.0.1',	'POST',	'/system/getServerInfo',	200,	200354862,	'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0',	'',	'',	'{\"code\":0,\"data\":{\"server\":{\"os\":{\"goos\":\"darwin\",\"numCpu\":16,\"compiler\":\"gc\",\"goVersion\":\"go1.22.0\",\"numGoroutine\":11},\"cpu\":{\"cpus\":[21.052631578733223,0,24.999999999772626,0,15.789473684109751,0,15.000000000272848,0,5.0000000000909495,0,0,0,0,0,0,0],\"cores\":8},\"ram\":{\"usedMb\":16747,\"totalMb\":32768,\"usedPercent\":51},\"disk\":{\"usedMb\":82112,\"usedGb\":80,\"totalMb\":953904,\"totalGb\":931,\"usedPercent\":8}}},\"msg\":\"获取成功\"}',	1),
(3,	'2024-04-23 11:21:40.906',	'2024-04-23 11:21:40.906',	NULL,	'127.0.0.1',	'POST',	'/system/getServerInfo',	200,	200340818,	'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0',	'',	'',	'{\"code\":0,\"data\":{\"server\":{\"os\":{\"goos\":\"darwin\",\"numCpu\":16,\"compiler\":\"gc\",\"goVersion\":\"go1.22.0\",\"numGoroutine\":11},\"cpu\":{\"cpus\":[20.000000000136424,0,15.789473683870412,0,14.999999999886313,0,9.999999999772626,0,0,0,0,0,0,0,0,0],\"cores\":8},\"ram\":{\"usedMb\":16749,\"totalMb\":32768,\"usedPercent\":51},\"disk\":{\"usedMb\":82112,\"usedGb\":80,\"totalMb\":953904,\"totalGb\":931,\"usedPercent\":8}}},\"msg\":\"获取成功\"}',	1),
(4,	'2024-04-23 11:21:50.906',	'2024-04-23 11:21:50.906',	NULL,	'127.0.0.1',	'POST',	'/system/getServerInfo',	200,	200889322,	'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0',	'',	'',	'{\"code\":0,\"data\":{\"server\":{\"os\":{\"goos\":\"darwin\",\"numCpu\":16,\"compiler\":\"gc\",\"goVersion\":\"go1.22.0\",\"numGoroutine\":11},\"cpu\":{\"cpus\":[23.809523809389756,0,15.789473684273512,0,14.999999999886313,0,5.000000000113687,0,5.0000000000909495,0,0,0,0,0,0,0],\"cores\":8},\"ram\":{\"usedMb\":16646,\"totalMb\":32768,\"usedPercent\":50},\"disk\":{\"usedMb\":82116,\"usedGb\":80,\"totalMb\":953904,\"totalGb\":931,\"usedPercent\":8}}},\"msg\":\"获取成功\"}',	1),
(5,	'2024-04-23 11:22:00.906',	'2024-04-23 11:22:00.906',	NULL,	'127.0.0.1',	'POST',	'/system/getServerInfo',	200,	201164098,	'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0',	'',	'',	'{\"code\":0,\"data\":{\"server\":{\"os\":{\"goos\":\"darwin\",\"numCpu\":16,\"compiler\":\"gc\",\"goVersion\":\"go1.22.0\",\"numGoroutine\":11},\"cpu\":{\"cpus\":[24.999999999772626,0,21.05263157911113,0,20.000000000363798,0,15.000000000272848,0,9.523809523974512,0,5.000000000113687,0,0,0,0,0],\"cores\":8},\"ram\":{\"usedMb\":16558,\"totalMb\":32768,\"usedPercent\":50},\"disk\":{\"usedMb\":82111,\"usedGb\":80,\"totalMb\":953904,\"totalGb\":931,\"usedPercent\":8}}},\"msg\":\"获取成功\"}',	1);

DROP TABLE IF EXISTS `sys_user_role`;
CREATE TABLE `sys_user_role` (
  `sys_user_id` bigint unsigned NOT NULL,
  `sys_role_role_id` bigint unsigned NOT NULL COMMENT '角色ID',
  PRIMARY KEY (`sys_user_id`,`sys_role_role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_user_role` (`sys_user_id`, `sys_role_role_id`) VALUES
(1,	888),
(1,	8881),
(1,	9528),
(2,	888);

DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_at` datetime(3) DEFAULT NULL,
  `updated_at` datetime(3) DEFAULT NULL,
  `deleted_at` datetime(3) DEFAULT NULL,
  `uuid` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户UUID',
  `username` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户登录名',
  `password` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户登录密码',
  `nick_name` varchar(191) COLLATE utf8mb4_general_ci DEFAULT '系统用户' COMMENT '用户昵称',
  `side_mode` varchar(191) COLLATE utf8mb4_general_ci DEFAULT 'dark' COMMENT '用户侧边主题',
  `header_img` varchar(191) COLLATE utf8mb4_general_ci DEFAULT 'https://qmplusimg.henrongyi.top/gva_header.jpg' COMMENT '用户头像',
  `base_color` varchar(191) COLLATE utf8mb4_general_ci DEFAULT '#fff' COMMENT '基础颜色',
  `active_color` varchar(191) COLLATE utf8mb4_general_ci DEFAULT '#1890ff' COMMENT '活跃颜色',
  `role_id` bigint unsigned DEFAULT '888' COMMENT '用户角色ID',
  `phone` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户手机号',
  `email` varchar(191) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户邮箱',
  `enable` bigint DEFAULT '1' COMMENT '用户是否被冻结 1正常 2冻结',
  PRIMARY KEY (`id`),
  KEY `idx_sys_user_username` (`username`),
  KEY `idx_sys_user_deleted_at` (`deleted_at`),
  KEY `idx_sys_user_uuid` (`uuid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `sys_user` (`id`, `created_at`, `updated_at`, `deleted_at`, `uuid`, `username`, `password`, `nick_name`, `side_mode`, `header_img`, `base_color`, `active_color`, `role_id`, `phone`, `email`, `enable`) VALUES
(1,	'2024-04-23 11:20:33.455',	'2024-04-23 11:20:33.461',	NULL,	'84d372e1-8c3f-4757-83f4-2ccbb8225c52',	'admin',	'$2a$10$LBo6sNlUu2ZbcuwqQZ10M.nPIxjc3H6zkuzpb4aoRPIa6Pn2uiUJO',	'Mr.奇淼',	'dark',	'https://qmplusimg.henrongyi.top/gva_header.jpg',	'#fff',	'#1890ff',	888,	'17611111111',	'333333333@qq.com',	1),
(2,	'2024-04-23 11:20:33.455',	'2024-04-23 11:20:33.468',	NULL,	'03b1b990-b486-4e36-ae7a-28634fab49f2',	'a303176530',	'$2a$10$KpgeobPL4Ili0gHeyuBc1OEb4PBU8r0DgsECjDPKogKW/5OGNE/vG',	'用户1',	'dark',	'https:///qmplusimg.henrongyi.top/1572075907logo.png',	'#fff',	'#1890ff',	9528,	'17611111111',	'333333333@qq.com',	1);

-- 2024-04-23 03:22:08
