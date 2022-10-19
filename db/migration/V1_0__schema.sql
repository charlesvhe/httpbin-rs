CREATE TABLE `config_meta` (
  `id` INTEGER PRIMARY KEY,
  `code` TEXT NOT NULL,
  `property` TEXT NOT NULL,
  `column_name` TEXT NOT NULL,
  `description` TEXT NOT NULL DEFAULT '',
  `sort` INTEGER NOT NULL DEFAULT '0',
  `gmt_create` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `gmt_modified` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE `config_item` (
  `id` INTEGER PRIMARY KEY,
  `meta_code` TEXT NOT NULL,
  `parent_id` INTEGER NOT NULL DEFAULT '0',
  `sort` INTEGER NOT NULL DEFAULT '0',
  `varchar1` TEXT NOT NULL DEFAULT '',
  `varchar2` TEXT NOT NULL DEFAULT '',
  `varchar3` TEXT NOT NULL DEFAULT '',
  `varchar4` TEXT NOT NULL DEFAULT '',
  `varchar5` TEXT NOT NULL DEFAULT '',
  `text1` TEXT,
  `text2` TEXT,
  `text3` TEXT,
  `decimal1` REAL NOT NULL DEFAULT '0.0000',
  `decimal2` REAL NOT NULL DEFAULT '0.0000',
  `decimal3` REAL NOT NULL DEFAULT '0.0000',
  `datetime1` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `datetime2` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `datetime3` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `gmt_create` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `gmt_modified` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO `config_meta` VALUES
(1,'userType','code','varchar1','类型编码',10,'2018-05-10 17:54:31','2018-05-10 21:14:08'),
(2,'userType','name','varchar2','类型名',20,'2018-05-10 17:54:31','2018-05-10 21:14:08'),
(3,'userType','description','varchar3','说明',30,'2018-05-10 17:54:31','2018-05-10 20:32:23'),
(4,'userType','img','varchar4','图标',40,'2018-05-10 17:54:31','2018-05-10 20:32:23');

INSERT INTO `config_meta` VALUES
(5,'sex','code','varchar1','性别编码',10,'2018-05-10 17:54:31','2018-05-10 21:14:08'),
(6,'sex','sex','varchar2','性别名',20,'2018-05-10 17:54:31','2018-05-10 21:14:08');

INSERT INTO `config_item`(`id`, `meta_code`, `varchar1`, `varchar2`, `varchar3`, `varchar4`, `gmt_create`, `gmt_modified`) VALUES
(1,'userType','user','普通用户','刚注册还未认证用户','1.jpg','2018-05-10 17:54:31','2018-05-10 21:14:08'),
(2,'userType','member','会员','已认证用户','2.jpg','2018-05-10 17:54:31','2018-05-10 21:14:08'),
(3,'userType','VIP1','铜牌会员','铜牌会员','3.jpg','2018-05-10 17:54:31','2018-05-10 21:14:08'),
(4,'userType','VIP2','银牌会员','银牌会员','4.jpg','2018-05-10 17:54:31','2018-05-10 21:14:08'),
(5,'userType','VIP3','金牌会员','金牌会员','5.jpg','2018-05-10 17:54:31','2018-05-10 21:14:08'),
(6,'userType','VVIP','VVIP','VVIP','6.jpg','2018-05-10 17:54:31','2018-05-10 21:14:08');

INSERT INTO `config_item`(`id`, `meta_code`, `varchar1`, `varchar2`,`gmt_create`, `gmt_modified`) VALUES
(7,'sex','Agender','无性别','2018-05-10 17:54:31','2018-05-10 21:14:08'),
(8,'sex','Male','男','2018-05-10 17:54:31','2018-05-10 21:14:08'),
(9,'sex','Female','女','2018-05-10 17:54:31','2018-05-10 21:14:08'),
(10,'sex','Bigender','双性','2018-05-10 17:54:31','2018-05-10 21:14:08');

