-- test.bot_users definition

CREATE TABLE `bot_users` IF NOT EXISTS  (
                             `id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
                             `username` varchar(255) NOT NULL,
                             `password` varchar(511) NOT NULL,
                             `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
                             PRIMARY KEY (`id`),
                             UNIQUE KEY `username` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- test.bot_project definition

CREATE TABLE `bot_project` (
                               `id` bigint NOT NULL AUTO_INCREMENT,
                               `owner_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '项目所有者id',
                               `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                               `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
                               `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT '项目描述',
                               `deleted` int DEFAULT '0' COMMENT '逻辑删除1删除，0未删除',
                               `remark` json DEFAULT NULL COMMENT '备注remark json，扩展字段',
                               `name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '项目名称',
                               `logo_url` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'logo url',
                               `slogan` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '广告语',
                               `bonus_pool` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '奖金池金额，实时计算得出',
                               `status` tinyint NOT NULL DEFAULT '1' COMMENT '1默认正常状态，保留字段（扩展其它状态）',
                               PRIMARY KEY (`id`),
                               UNIQUE KEY `bot_project_name_IDX` (`name`),
                               KEY `bot_project_bonus_pool_IDX` (`bonus_pool`) USING BTREE,
                               KEY `bot_project_owner_id_IDX` (`owner_id`) USING BTREE,
                               KEY `bot_project_status_IDX` (`status`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='bot 项目表';

-- test.bot_task definition

CREATE TABLE `bot_task` (
                            `id` bigint NOT NULL AUTO_INCREMENT,
                            `project_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'bot project id',
                            `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                            `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
                            `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT '描述',
                            `deleted` int DEFAULT '0' COMMENT '逻辑删除1删除，0未删除',
                            `remark` json DEFAULT NULL COMMENT '备注remark json，扩展字段',
                            `name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '任务名称',
                            `end_time` datetime DEFAULT NULL COMMENT '任务结束时间，null长期有效不结束',
                            `sync` tinyint NOT NULL DEFAULT '1' COMMENT '同步/异步，1默认同步',
                            `polling` tinyint NOT NULL DEFAULT '0' COMMENT '任务轮巡，0默认不轮巡',
                            `start_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '任务开始时间',
                            `type` tinyint NOT NULL DEFAULT '0' COMMENT '任务类型，0默认，其它如拉新、社交、游戏等',
                            `status` tinyint NOT NULL DEFAULT '1' COMMENT '1默认正常状态，保留字段（扩展其它状态）',
                            PRIMARY KEY (`id`),
                            KEY `bot_task_name_IDX` (`name`) USING BTREE,
                            KEY `bot_task_project_id_IDX` (`project_id`) USING BTREE,
                            KEY `bot_task_status_IDX` (`status`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='bot 项目任务表';


-- test.bot_action definition

CREATE TABLE `bot_action` (
                              `id` bigint NOT NULL AUTO_INCREMENT,
                              `project_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'bot project id',
                              `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                              `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
                              `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT '描述',
                              `deleted` int DEFAULT '0' COMMENT '逻辑删除1删除，0未删除',
                              `remark` json DEFAULT NULL COMMENT '备注remark json，扩展字段',
                              `name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '任务名称',
                              `end_time` datetime DEFAULT NULL COMMENT '结束时间，null长期有效不结束,默认和任务一致',
                              `task_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'bot task id',
                              `category_code` tinyint NOT NULL DEFAULT '0' COMMENT '0',
                              `start_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '开始时间，默认和任务一直',
                              `params` json DEFAULT NULL COMMENT 'action参数 json {...}',
                              `required_action_ids` json DEFAULT NULL COMMENT 'array of pre-request action ids [1,2,...]',
                              `status` tinyint NOT NULL DEFAULT '1' COMMENT '1默认正常状态，保留字段（扩展其它状态）',
                              PRIMARY KEY (`id`),
                              KEY `bot_action_category_code_IDX` (`category_code`) USING BTREE,
                              KEY `bot_action_name_IDX` (`name`) USING BTREE,
                              KEY `bot_action_project_id_IDX` (`project_id`) USING BTREE,
                              KEY `bot_action_status_IDX` (`status`) USING BTREE,
                              KEY `bot_action_task_id_IDX` (`task_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='bot 任务行为表';


-- test.bot_user_action definition

CREATE TABLE `bot_user_action` (
                                   `id` bigint NOT NULL AUTO_INCREMENT,
                                   `project_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'bot project id',
                                   `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                                   `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                   `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT '描述',
                                   `deleted` int DEFAULT '0' COMMENT '逻辑删除1删除，0未删除',
                                   `remark` json DEFAULT NULL COMMENT '备注remark json，扩展字段',
                                   `action_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'action id',
                                   `end_time` datetime DEFAULT NULL COMMENT '结束时间，null长期有效不结束,默认和任务一致',
                                   `task_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'bot task id',
                                   `category_code` tinyint NOT NULL DEFAULT '0' COMMENT '0',
                                   `start_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '开始时间，默认和任务一直',
                                   `params` json DEFAULT NULL COMMENT 'action参数 json {...}',
                                   `required_action_ids` json DEFAULT NULL COMMENT 'array of pre-request action ids [1,2,...]',
                                   `project_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'bot project name',
                                   `task_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'bot task name',
                                   `action_id` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'action名称',
                                   `res_data` json DEFAULT NULL COMMENT 'action 返回数据  json {}',
                                   `user_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'user id in action',
                                   `status` tinyint NOT NULL DEFAULT '1' COMMENT '1默认正常状态，保留字段（扩展其它状态）',
                                   PRIMARY KEY (`id`),
                                   KEY `bot_user_action_action_id_IDX` (`action_id`) USING BTREE,
                                   KEY `bot_user_action_action_name_IDX` (`action_name`) USING BTREE,
                                   KEY `bot_user_action_category_code_IDX` (`category_code`) USING BTREE,
                                   KEY `bot_user_action_project_id_IDX` (`project_id`) USING BTREE,
                                   KEY `bot_user_action_project_name_IDX` (`project_name`) USING BTREE,
                                   KEY `bot_user_action_status_IDX` (`status`) USING BTREE,
                                   KEY `bot_user_action_task_id_IDX` (`task_id`) USING BTREE,
                                   KEY `bot_user_action_task_name_IDX` (`task_name`) USING BTREE,
                                   KEY `bot_user_action_user_id_IDX` (`user_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='bot 用户任务行为表';
