# IM SDK API

## 设备

#### 初始化

**init**

> Requests and Parameter Description
>
> | Parameter      | Type   | Must | Parameter Description          |
> | :------------- | :----- | :--- | :----------------------------- |
> | account_pubkey | String | yes  | 账户公钥                       |
> | device_pubkey  | String | yes  | 设备公钥                       |
> | proof          | String | yes  | 证明数据，用于验证设备的合法性 |
>
> Response and Parameters in data
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | encrypt_data | String | yes  | 加密信息              |

------



#### 解密

**decrypt**

> Requests and Parameter Description
>
> | Parameter      | Type   | Must | Parameter Description |
> | :------------- | :----- | :--- | :-------------------- |
> | encrypt_data   | String | yes  | 加密信息              |
> | account_prikey | String | yes  | 账号私钥              |
> | passwd         | String | yes  | 授权密码              |
>
> Response and Parameters in data
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | decrypt_data | String | yes  | 解密信息              |

------



#### 授权

**warrant**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | decrypt_data | String | yes  | 解密信息              |
>
> Response and Parameters in data
>
> | Parameter         | Type   | Must | Parameter Description |
> | :---------------- | :----- | :--- | :-------------------- |
> | server_public_key | String | yes  | 服务端返回的公钥      |

------



#### 更新设备令牌

**update_token**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | token     | String | yes  | 访问令牌              |
>
> Response and Parameters in data
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | token     | String | yes  | 新令牌                |

------



#### 删除设备

**del_device**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | device_id | int  | yes  | 设备id                |
>
> Response and Parameters in data
>
> no data

------



## 账号

#### 更新账号信息

**update_info**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | 用户id                |
> | bio       | String | yes  | 简介                  |
> | name      | String | yes  | 名称                  |
> | gender    | int    | yes  | 性别                  |
>
> Response and Parameters in data
>
> no data

------



#### 更新头像

**update_avatar**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | 用户id                |
> | avatar    | String | yes  | 头像数据              |
>
> Response and Parameters in data
>
> no data

------



#### 加入社区

**add_community**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | user_id      | int    | yes  | 用户id                |
> | community_id | int    | yes  | 社区id                |
> | content      | String | yes  | 入组信息              |
>
> Response and Parameters in data
>
> no data

------

#### 退出社区

**quit_community**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | user_id      | int  | yes  | 用户id                |
> | community_id | int  | yes  | 社区id                |
>
> Response and Parameters in data
>
> no data

------



#### 添加联系人

**add_contact**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | 用户id                |
> | friend_id | int    | yes  | 联系人id              |
> | content   | String | no   | 发送信息              |
>
> Response and Parameters in data
>
> no data

------



## 会话

#### 发送消息

**push_msg**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | 接收者                |
> | from_id   | int    | yes  | 发送者                |
> | content   | String | yes  | 消息内容              |
> | mode      | int    | yes  | 消息类型              |
> | chat_type | int    | yes  | 聊天类型              |
>
> Response and Parameters in data
>
> no data

------

#### 拉取消息

**pull_msg**

> Requests and Parameter Description
>
> | Parameter  | Type | Must | Parameter Description |
> | :--------- | :--- | :--- | :-------------------- |
> | message_id | int  | yes  | 消息id                |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | message表数据结构（后续补充） |      |      |                       |

------



#### 更新消息

**update_msg**

> Requests and Parameter Description
>
> | Parameter  | Type   | Must | Parameter Description |
> | :--------- | :----- | :--- | :-------------------- |
> | message_id | int    | yes  | 消息id                |
> | content    | String | yes  | 消息内容              |
>
> Response and Parameters in data
>
> no data

------



#### 删除消息

**del_msg**

> Requests and Parameter Description
>
> | Parameter  | Type | Must | Parameter Description |
> | :--------- | :--- | :--- | :-------------------- |
> | message_id | int  | yes  | 消息id                |
>
> Response and Parameters in data
>
> no data

------



#### 置顶消息

**pin_msg**

> Requests and Parameter Description
>
> | Parameter  | Type | Must | Parameter Description |
> | :--------- | :--- | :--- | :-------------------- |
> | message_id | int  | yes  | 消息id                |
>
> Response and Parameters in data
>
> no data

------



#### 聊天列表

**chat_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                  | Type | Must | Parameter Description |
> | :------------------------- | :--- | :--- | :-------------------- |
> | chat表数据结构（后续补充） |      |      |                       |

------



#### 聊天记录搜索

**chat_detail**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | chat_id   | int    | yes  | 聊天id                |
> | keyword   | String | yes  | 关键字                |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | message表数据结构（后续补充） |      |      |                       |

------



## 精灵

#### 创建精灵

**create_elf**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | name      | String | yes  | 名称                  |
> | avatar    | String | yes  | 头像                  |
> | type      | int    | yes  | 1/普通                |
>
> Response and Parameters in data
>
> no data

------

#### 更新精灵

**update_elf**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | elf_id    | int    | yes  | 精灵id                |
> | name      | String | yes  | 名称                  |
> | avatar    | String | yes  | 头像                  |
> | type      | int    | yes  | 1/普通                |
> | status    | int    | yes  | 状态                  |
>
> Response and Parameters in data
>
> no data

------



#### 精灵信息

**elf_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | elf_id    | int  | yes  | 精灵id                |
>
> Response and Parameters in data
>
> | Parameter                 | Type | Must | Parameter Description |
> | :------------------------ | :--- | :--- | :-------------------- |
> | elf表数据结构（后续补充） |      |      |                       |

------



#### 删除精灵

**del_elf**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | elf_id    | int  | yes  | 精灵id                |
>
> Response and Parameters in data
>
> no data

------



## 通讯录

#### 搜索

**search_contact**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | keyword   | String | yes  | 关键字                |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | contact表数据结构（后续补充） |      |      |                       |

------



#### 获取联系人详情

****

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | contact表数据结构（后续补充） |      |      |                       |

------



#### 获取通讯录列表

**contact_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | contact表数据结构（后续补充） |      |      |                       |

------



#### 更新联系人信息

**update_contact**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | 用户id                |
> | friend_id | int    | yes  | 联系人id              |
> | remark    | String | yes  | 备注                  |
>
> Response and Parameters in data
>
> no data

------



#### 删除联系人

**del_contact**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
> | friend_id | int  | yes  | 联系人id              |
>
> Response and Parameters in data
>
> no data

------



## 社区

#### 获取社区列表

**community_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                       | Type | Must | Parameter Description |
> | :------------------------------ | :--- | :--- | :-------------------- |
> | community表数据结构（后续补充） |      |      |                       |

------



#### 社区详情

**community_detail**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
>
> Response and Parameters in data
>
> | Parameter                       | Type | Must | Parameter Description |
> | :------------------------------ | :--- | :--- | :-------------------- |
> | community表数据结构（后续补充） |      |      |                       |

------



#### 更新社区信息

**update_community**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | community_id | int    | yes  | 社区id                |
> | name         | String | yes  | 名称                  |
> | bio          | String | yes  | 简介                  |
> | passwd       | String | no   | 密码                  |
> | announcement | String | no   | 公告                  |
> | pinned       | bool   | yes  | 置顶                  |
> | status       | int    | yes  | 状态                  |
>
> Response and Parameters in data
>
> no data

------



#### 删除社区

**del_community**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | id        | int  | yes  | 社区id                |
>
> Response and Parameters in data
>
> no data

------



#### 创建社区

**create_community**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | 用户id                |
> | father_id | int    | no   | 父社区id              |
> | name      | String | yes  | 社区名称              |
> | bio       | String | yes  | 简介                  |
> | passwd    | String | no   | 密码                  |
> | status    | int    | yes  | 1/普通社区 2/二级社区 |
> 
>Response and Parameters in data
> 
>| Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |

------



### 社区帖子

#### 获取社区帖子列表

**post_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | id        | int  | yes  | 社区id                |
>
> Response and Parameters in data
>
> | Parameter                            | Type | Must | Parameter Description |
> | :----------------------------------- | :--- | :--- | :-------------------- |
> | community_post表数据结构（后续补充） |      |      |                       |

------



#### 发布新帖子

**create_post**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | community_id | int    | yes  | 社区id                |
> | user_id      | int    | yes  | 用户id                |
> | name         | String | yes  | 名称                  |
> | content      | String | yes  | 内容                  |
>
> Response and Parameters in data
>
> no data

------



#### 更新新帖子

**update_post**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | post_id   | int    | yes  | 帖子id                |
> | name      | String | yes  | 名称                  |
> | content   | String | yes  | 内容                  |
>
> Response and Parameters in data
>
> no data

------



#### 查看帖子详情

**post_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | post_id   | int  | yes  | 帖子id                |
>
> Response and Parameters in data
>
> | Parameter                            | Type | Must | Parameter Description |
> | :----------------------------------- | :--- | :--- | :-------------------- |
> | community_post表数据结构（后续补充） |      |      |                       |

------

#### 删除帖子

**del_post**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | post_id   | int  | yes  | 帖子id                |
>
> Response and Parameters in data
>
> no data

------



#### 获取社区帖子回复列表

**post_reply_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | post_id   | int  | yes  | 社区id                |
>
> Response and Parameters in data
>
> | Parameter                                  | Type | Must | Parameter Description |
> | :----------------------------------------- | :--- | :--- | :-------------------- |
> | community_post_reply表数据结构（后续补充） |      |      |                       |

------



#### 回复帖子

**reply_post**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | post_id   | int    | yes  | 帖子id                |
> | user_id   | int    | yes  | 用户id                |
> | content   | String | yes  | 内容                  |
>
> Response and Parameters in data
>
> no data

------



#### 更新帖子回复

**update_post_reply**

> Requests and Parameter Description
>
> | Parameter     | Type   | Must | Parameter Description |
> | :------------ | :----- | :--- | :-------------------- |
> | post_reply_id | int    | yes  | 帖子id                |
> | content       | String | yes  | 内容                  |
>
> Response and Parameters in data
>
> no data

------

#### 删除帖子回复

**del_post_reply**

> Requests and Parameter Description
>
> | Parameter     | Type | Must | Parameter Description |
> | :------------ | :--- | :--- | :-------------------- |
> | post_reply_id | int  | yes  | 帖子id                |
>
> Response and Parameters in data
>
> no data

------



#### 获取社区帖子回复列表

**post_reply_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | post_id   | int  | yes  | 社区id                |
>
> Response and Parameters in data
>
> | Parameter                                  | Type | Must | Parameter Description |
> | :----------------------------------------- | :--- | :--- | :-------------------- |
> | community_post_reply表数据结构（后续补充） |      |      |                       |

------





### 社区管理员

#### 管理员列表

**admin_list**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
>
> Response and Parameters in data
>
> | Parameter                             | Type | Must | Parameter Description |
> | :------------------------------------ | :--- | :--- | :-------------------- |
> | community_admin表数据结构（后续补充） |      |      |                       |

------

#### 添加管理员

**add_admin**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | community_id | int    | yes  | 社区id                |
> | user_id      | int    | yes  | 用户id                |
> | type         | String | yes  | 类型                  |
>
> Response and Parameters in data
>
> no data

------



#### 更新管理员信息

**update_admin**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
> | user_id      | int  | yes  | 用户id                |
> | type         | int  | yes  | 类型                  |
>
> Response and Parameters in data
>
> no data

------



#### 删除管理员

**del_admin**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
> | user_id      | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> no data

------



#### 管理员详情

**admin_detail**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
> | user_id      | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                             | Type | Must | Parameter Description |
> | :------------------------------------ | :--- | :--- | :-------------------- |
> | community_admin表数据结构（后续补充） |      |      |                       |

------



### 社区成员

#### 成员列表

**member_list**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
>
> Response and Parameters in data
>
> | Parameter                              | Type | Must | Parameter Description |
> | :------------------------------------- | :--- | :--- | :-------------------- |
> | community_member表数据结构（后续补充） |      |      |                       |

------

#### 添加成员

**add_member**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
> | user_id      | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> no data

------



#### 更新成员信息

**update_member**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
> | user_id      | int  | yes  | 用户id                |
> | type         | int  | yes  | 类型                  |
>
> Response and Parameters in data
>
> no data

------



#### 删除成员

**del_member**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
> | user_id      | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> no data

------



#### 成员详情

**member_detail**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | 社区id                |
> | user_id      | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                              | Type | Must | Parameter Description |
> | :------------------------------------- | :--- | :--- | :-------------------- |
> | community_member表数据结构（后续补充） |      |      |                       |

------



## 申请

#### 获取申请列表

**apply_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                   | Type | Must | Parameter Description |
> | :-------------------------- | :--- | :--- | :-------------------- |
> | apply表数据结构（后续补充） |      |      |                       |

------



#### 查看申请详情

**applly_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | apply_id  | int  | yes  | 申请id                |
>
> Response and Parameters in data
>
> | Parameter                   | Type | Must | Parameter Description |
> | :-------------------------- | :--- | :--- | :-------------------- |
> | apply表数据结构（后续补充） |      |      |                       |

------

#### 回复申请

**reply_apply**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description                        |
> | :-------- | :----- | :--- | :------------------------------------------- |
> | apply_id  | int    | yes  | 申请id                                       |
> | user_id   | int    | yes  | 用户id                                       |
> | content   | String | no   | 回复内容                                     |
> | status    | int    | yes  | 审核结果：1/通过 2/回复 3/拒绝 7/请求 8/过期 |
>
> Response and Parameters in data
>
> no data

------

#### 删除申请

**del_apply**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | apply_id  | int  | yes  | 申请id                |
>
> Response and Parameters in data
>
> no data

------



## 设置

#### 获取设置信息

**settings_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                      | Type | Must | Parameter Description |
> | :----------------------------- | :--- | :--- | :-------------------- |
> | settings表数据结构（后续补充） |      |      |                       |

------



#### 修改语言

**update_language**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | 用户id                |
> | language  | String | no   | 语言                  |
>
> Response and Parameters in data
>
> no data

------



### 收藏夹

#### 收藏夹列表

**favorite_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                      | Type | Must | Parameter Description |
> | :----------------------------- | :--- | :--- | :-------------------- |
> | favorite表数据结构（后续补充） |      |      |                       |

------



#### 收藏项详情

**favorite_detail**

> Requests and Parameter Description
>
> | Parameter   | Type | Must | Parameter Description |
> | :---------- | :--- | :--- | :-------------------- |
> | favorite_id | int  | yes  | 收藏项id              |
>
> Response and Parameters in data
>
> | Parameter                      | Type | Must | Parameter Description |
> | :----------------------------- | :--- | :--- | :-------------------- |
> | favorite表数据结构（后续补充） |      |      |                       |

------



#### 添加到收藏夹

**add_favorite**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | 用户id                |
> | content   | String | yes  | 内容                  |
>
> Response and Parameters in data
>
> no data

------

#### 从收藏夹移除

**del_favorite**

> Requests and Parameter Description
>
> | Parameter   | Type | Must | Parameter Description |
> | :---------- | :--- | :--- | :-------------------- |
> | favorite_id | int  | yes  | 收藏项id              |
>
> Response and Parameters in data
>
> no data

------



## 导航

#### 导航列表

**nav_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | 用户id                |
>
> Response and Parameters in data
>
> | Parameter                 | Type | Must | Parameter Description |
> | :------------------------ | :--- | :--- | :-------------------- |
> | nav表数据结构（后续补充） |      |      |                       |

------

#### 添加导航

**add_nav**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description       |
> | :-------- | :--- | :--- | :-------------------------- |
> | user_id   | int  | yes  | 用户id                      |
> | type      | int  | yes  | 1/私聊 2/社区 3/精灵 7/广播 |
> | type_id   | int  | yes  | 对应的id                    |
> | sort      | int  | yes  | 位置索引                    |
>
> Response and Parameters in data
>
> no data

------

#### 更新导航

**update_nav**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | nav_id    | int  | yes  | 导航id                |
> | sort      | int  | no   | 位置索引              |
>
> Response and Parameters in data
>
> no data

------



#### 删除导航

**del_nav**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | nav_id    | int  | yes  | 导航id                |
>
> Response and Parameters in data
>
> no data

------

