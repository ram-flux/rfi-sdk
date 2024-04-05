# Ram Flux SDK API

## Device

#### Initialization

**init**

> Requests and Parameter Description
>
> | Parameter      | Type   | Must | Parameter Description          |
> | :------------- | :----- | :--- | :----------------------------- |
> | account_pubkey | String | yes  | Account public key             |
> | device_pubkey  | String | yes  | Device public key              |
> | proof          | String | yes  | Proof data for verifying device authenticity |
>
> Response and Parameters in data
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | encrypt_data | String | yes  | Encrypted information |

------

#### Decryption

**decrypt**

> Requests and Parameter Description
>
> | Parameter      | Type   | Must | Parameter Description |
> | :------------- | :----- | :--- | :-------------------- |
> | encrypt_data   | String | yes  | Encrypted information |
> | account_prikey | String | yes  | Account private key   |
> | passwd         | String | yes  | Authorization password|
>
> Response and Parameters in data
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | decrypt_data | String | yes  | Decrypted information |

------

#### Authorization

**warrant**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | decrypt_data | String | yes  | Decrypted information |
>
> Response and Parameters in data
>
> | Parameter         | Type   | Must | Parameter Description |
> | :---------------- | :----- | :--- | :-------------------- |
> | server_public_key | String | yes  | Server returned public key |

------

#### Update Device Token

**update_token**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | token     | String | yes  | Access token         |
>
> Response and Parameters in data
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | token     | String | yes  | New token             |

------

#### Delete Device

**del_device**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | device_id | int  | yes  | Device ID             |
>
> Response and Parameters in data
>
> no data

------

## Account

#### Update Account Information

**update_info**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | User ID               |
> | bio       | String | yes  | Bio                   |
> | name      | String | yes  | Name                  |
> | gender    | int    | yes  | Gender                |
>
> Response and Parameters in data
>
> no data

------

#### Update Avatar

**update_avatar**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | User ID               |
> | avatar    | String | yes  | Avatar data           |
>
> Response and Parameters in data
>
> no data

------

#### Account Detail

**account_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | Account table structure (to be added later) |      |      |                       |

------

#### Join Community

**add_community**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | user_id      | int    | yes  | User ID               |
> | community_id | int    | yes  | Community ID          |
> | content      | String | yes  | Joining message       |
>
> Response and Parameters in data
>
> no data

------

#### Leave Community

**quit_community**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |


> | user_id      | int  | yes  | User ID               |
> | community_id | int  | yes  | Community ID          |
>
> Response and Parameters in data
>
> no data

------

#### Add Contact

**add_contact**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | User ID               |
> | friend_id | int    | yes  | Contact ID            |
> | content   | String | no   | Message               |
>
> Response and Parameters in data
>
> no data

------

## Conversation

#### Send Message

**push_msg**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | Recipient             |
> | from_id   | int    | yes  | Sender                |
> | content   | String | yes  | Message content       |
> | mode      | int    | yes  | Message type          |
> | chat_type | int    | yes  | Chat type             |
>
> Response and Parameters in data
>
> no data

------

#### Pull Messages

**pull_msg**

> Requests and Parameter Description
>
> | Parameter  | Type | Must | Parameter Description |
> | :--------- | :--- | :--- | :-------------------- |
> | message_id | int  | yes  | Message ID            |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | Message table structure (to be added later) |      |      |                       |

------

#### Update Message

**update_msg**

> Requests and Parameter Description
>
> | Parameter  | Type   | Must | Parameter Description |
> | :--------- | :----- | :--- | :-------------------- |
> | message_id | int    | yes  | Message ID            |
> | content    | String | yes  | Message content       |
>
> Response and Parameters in data
>
> no data

------

#### Delete Message

**del_msg**

> Requests and Parameter Description
>
> | Parameter  | Type | Must | Parameter Description |
> | :--------- | :--- | :--- | :-------------------- |
> | message_id | int  | yes  | Message ID            |
>
> Response and Parameters in data
>
> no data

------

#### Pin Message

**pin_msg**

> Requests and Parameter Description
>
> | Parameter  | Type | Must | Parameter Description |
> | :--------- | :--- | :--- | :-------------------- |
> | message_id | int  | yes  | Message ID            |
>
> Response and Parameters in data
>
> no data

------

#### Chat List

**chat_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                  | Type | Must | Parameter Description |
> | :------------------------- | :--- | :--- | :-------------------- |
> | Chat table structure (to be added later) |      |      |                       |

------

#### Search Chat History

**chat_detail**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | chat_id   | int    | yes  | Chat ID               |
> | keyword   | String | yes  | Keyword               |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | Message table structure (to be added later) |      |      |                       |

------

## Sprite

#### Create Sprite

**create_elf**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | name      | String | yes  | Name                  |
> | avatar    | String | yes  | Avatar                |
> | type      | int    | yes  | 1/Normal              |
>
> Response and Parameters in data
>
> no data

------

#### Update Sprite

**update_elf**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | elf_id    | int    | yes  | Sprite ID             |
> | name      | String | yes  | Name                  |
> | avatar

    | String | yes  | Avatar                |
> | type      | int    | yes  | 1/Normal              |
> | status    | int    | yes  | Status                |
>
> Response and Parameters in data
>
> no data

------

#### Sprite Information

**elf_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | elf_id    | int  | yes  | Sprite ID             |
>
> Response and Parameters in data
>
> | Parameter                 | Type | Must | Parameter Description |
> | :------------------------ | :--- | :--- | :-------------------- |
> | Elf table structure (to be added later) |      |      |                       |

------

#### Delete Sprite

**del_elf**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | elf_id    | int  | yes  | Sprite ID             |
>
> Response and Parameters in data
>
> no data

------

## Address Book

#### Search

**search_contact**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | keyword   | String | yes  | Keyword               |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | Contact table structure (to be added later) |      |      |                       |

------



#### Get Contact Details

****

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | Contact table structure (to be added later) |      |      |                       |

------

#### Get Address Book List

**contact_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                     | Type | Must | Parameter Description |
> | :---------------------------- | :--- | :--- | :-------------------- |
> | Contact table structure (to be added later) |      |      |                       |

------

#### Update Contact Information

**update_contact**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | User ID               |
> | friend_id | int    | yes  | Contact ID            |
> | remark    | String | yes  | Remark                |
>
> Response and Parameters in data
>
> no data

------

#### Delete Contact

**del_contact**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
> | friend_id | int  | yes  | Contact ID            |
>
> Response and Parameters in data
>
> no data

------

## Community

#### Get Community List

**community_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                       | Type | Must | Parameter Description |
> | :------------------------------ | :--- | :--- | :-------------------- |
> | Community table structure (to be added later) |      |      |                       |

------

#### Community Detail

**community_detail**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
>
> Response and Parameters in data
>
> | Parameter                       | Type | Must | Parameter Description |
> | :------------------------------ | :--- | :--- | :-------------------- |
> | Community table structure (to be added later) |      |      |                       |

------

#### Update Community Information

**update_community**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> |

 community_id | int    | yes  | Community ID          |
> | name         | String | yes  | Name                  |
> | bio          | String | yes  | Bio                   |
> | passwd       | String | no   | Password              |
> | announcement | String | no   | Announcement          |
> | pinned       | bool   | yes  | Pinned                |
> | status       | int    | yes  | Status                |
>
> Response and Parameters in data
>
> no data

------

#### Delete Community

**del_community**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | id        | int  | yes  | Community ID          |
>
> Response and Parameters in data
>
> no data

------

#### Create Community

**create_community**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | User ID               |
> | father_id | int    | no   | Parent community ID   |
> | name      | String | yes  | Community name        |
> | bio       | String | yes  | Bio                   |
> | passwd    | String | no   | Password              |
> | status    | int    | yes  | 1/Normal community 2/Sub-community |
> 
>Response and Parameters in data
> 
>| Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |

------

### Community Posts

#### Get Community Post List

**post_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | id        | int  | yes  | Community ID          |
>
> Response and Parameters in data
>
> | Parameter                            | Type | Must | Parameter Description |
> | :----------------------------------- | :--- | :--- | :-------------------- |
> | Community post table structure (to be added later) |      |      |                       |

------

#### Publish New Post

**create_post**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | community_id | int    | yes  | Community ID          |
> | user_id      | int    | yes  | User ID               |
> | name         | String | yes  | Name                  |
> | content      | String | yes  | Content               |
>
> Response and Parameters in data
>
> no data

------

#### Update Post

**update_post**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | post_id   | int    | yes  | Post ID               |
> | name      | String | yes  | Name                  |
> | content   | String | yes  | Content               |
>
> Response and Parameters in data
>
> no data

------

#### View Post Details

**post_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | post_id   | int  | yes  | Post ID               |
>
> Response and Parameters in data
>
> | Parameter                            | Type | Must | Parameter Description |
> | :----------------------------------- | :--- | :--- | :-------------------- |
> | Community post table structure (to be added later) |      |      |                       |

------

#### Delete Post

**del_post**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | post_id   | int  | yes  | Post ID               |
>
> Response and Parameters in data
>
> no data

------

#### Get Community Post Reply List

**post_reply_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | post_id   | int  | yes  | Community ID          |
>
> Response and Parameters in data
>
> | Parameter                                  | Type | Must | Parameter Description |
> | :----------------------------------------- | :--- | :--- | :-------------------- |
> | Community post reply table structure (to be added later) |      |      |                       |

------

#### Reply to Post

**reply_post**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :

----- | :--- | :-------------------- |
> | post_id   | int    | yes  | Post ID               |
> | user_id   | int    | yes  | User ID               |
> | content   | String | yes  | Content               |
>
> Response and Parameters in data
>
> no data

------

#### Update Post Reply

**update_post_reply**

> Requests and Parameter Description
>
> | Parameter     | Type   | Must | Parameter Description |
> | :------------ | :----- | :--- | :-------------------- |
> | post_reply_id | int    | yes  | Post ID               |
> | content       | String | yes  | Content               |
>
> Response and Parameters in data
>
> no data

------

#### Delete Post Reply

**del_post_reply**

> Requests and Parameter Description
>
> | Parameter     | Type | Must | Parameter Description |
> | :------------ | :--- | :--- | :-------------------- |
> | post_reply_id | int  | yes  | Post ID               |
>
> Response and Parameters in data
>
> no data

------

#### Get Community Post Reply List

**post_reply_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | post_id   | int  | yes  | Community ID          |
>
> Response and Parameters in data
>
> | Parameter                                  | Type | Must | Parameter Description |
> | :----------------------------------------- | :--- | :--- | :-------------------- |
> | Community post reply table structure (to be added later) |      |      |                       |

------





### Community Administrators

#### Administrator List

**admin_list**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
>
> Response and Parameters in data
>
> | Parameter                             | Type | Must | Parameter Description |
> | :------------------------------------ | :--- | :--- | :-------------------- |
> | Community admin table structure (to be added later) |      |      |                       |

------

#### Add Administrator

**add_admin**

> Requests and Parameter Description
>
> | Parameter    | Type   | Must | Parameter Description |
> | :----------- | :----- | :--- | :-------------------- |
> | community_id | int    | yes  | Community ID          |
> | user_id      | int    | yes  | User ID               |
> | type         | String | yes  | Type                  |
>
> Response and Parameters in data
>
> no data

------

#### Update Administrator Information

**update_admin**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
> | user_id      | int  | yes  | User ID               |
> | type         | int  | yes  | Type                  |
>
> Response and Parameters in data
>
> no data

------

#### Delete Administrator

**del_admin**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
> | user_id      | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> no data

------

#### Administrator Details

**admin_detail**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
> | user_id      | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                             | Type | Must | Parameter Description |
> | :------------------------------------ | :--- | :--- | :-------------------- |
> | Community admin table structure (to be added later) |      |      |                       |

------

### Community Members

#### Member List

**member_list**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
>
> Response and Parameters in data
>
> | Parameter                              | Type | Must | Parameter Description |
> | :------------------------------------- | :--- | :--- | :-------------------- |
> | Community member table structure (to be added later) |      |      |                       |

------

#### Add Member

**add_member**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :-----------

 | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
> | user_id      | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> no data

------

#### Update Member Information

**update_member**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
> | user_id      | int  | yes  | User ID               |
> | type         | int  | yes  | Type                  |
>
> Response and Parameters in data
>
> no data

------

#### Delete Member

**del_member**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
> | user_id      | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> no data

------

#### Member Details

**member_detail**

> Requests and Parameter Description
>
> | Parameter    | Type | Must | Parameter Description |
> | :----------- | :--- | :--- | :-------------------- |
> | community_id | int  | yes  | Community ID          |
> | user_id      | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                              | Type | Must | Parameter Description |
> | :------------------------------------- | :--- | :--- | :-------------------- |
> | Community member table structure (to be added later) |      |      |                       |

------

## Application

#### Create Application

**create_apply**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description        |
> | :-------- | :----- | :--- | :--------------------------- |
> | type      | int    | yes  | 1/Add friend 2/Apply to join community |
> | type_id   | int    | yes  | 2/Apply to join community    |
> | user_id   | int    | yes  | Requester                    |
> | content   | String | yes  | Request message              |
>
> Response and Parameters in data
>
> no data

------

#### Get Application List

**apply_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                   | Type | Must | Parameter Description |
> | :-------------------------- | :--- | :--- | :-------------------- |
> | Apply table structure (to be added later) |      |      |                       |

------

#### View Application Details

**applly_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | apply_id  | int  | yes  | Application ID        |
>
> Response and Parameters in data
>
> | Parameter                   | Type | Must | Parameter Description |
> | :-------------------------- | :--- | :--- | :-------------------- |
> | Apply table structure (to be added later) |      |      |                       |

------

#### Reply to Application

**reply_apply**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description                      |
> | :-------- | :----- | :--- | :----------------------------------------- |
> | apply_id  | int    | yes  | Application ID                             |
> | user_id   | int    | yes  | User ID                                   |
> | content   | String | no   | Reply content                             |
> | status    | int    | yes  | Review result: 1/Approved 2/Reply 3/Rejected 7/Request 8/Expired |
>
> Response and Parameters in data
>
> no data

------

#### Delete Application

**del_apply**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | apply_id  | int  | yes  | Application ID        |
>
> Response and Parameters in data
>
> no data

------

## Settings

#### Get Settings Information

**settings_detail**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                      | Type | Must | Parameter Description |
> | :-----------------------------

 | :--- | :--- | :-------------------- |
> | Settings table structure (to be added later) |      |      |                       |

------

#### Change Language

**update_language**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | User ID               |
> | language  | String | no   | Language              |
>
> Response and Parameters in data
>
> no data

------

### Favorites

#### Favorites List

**favorite_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                      | Type | Must | Parameter Description |
> | :----------------------------- | :--- | :--- | :-------------------- |
> | Favorite table structure (to be added later) |      |      |                       |

------

#### Favorite Item Details

**favorite_detail**

> Requests and Parameter Description
>
> | Parameter   | Type | Must | Parameter Description |
> | :---------- | :--- | :--- | :-------------------- |
> | favorite_id | int  | yes  | Favorite item ID      |
>
> Response and Parameters in data
>
> | Parameter                      | Type | Must | Parameter Description |
> | :----------------------------- | :--- | :--- | :-------------------- |
> | Favorite table structure (to be added later) |      |      |                       |

------

#### Add to Favorites

**add_favorite**

> Requests and Parameter Description
>
> | Parameter | Type   | Must | Parameter Description |
> | :-------- | :----- | :--- | :-------------------- |
> | user_id   | int    | yes  | User ID               |
> | content   | String | yes  | Content               |
>
> Response and Parameters in data
>
> no data

------

#### Remove from Favorites

**del_favorite**

> Requests and Parameter Description
>
> | Parameter   | Type | Must | Parameter Description |
> | :---------- | :--- | :--- | :-------------------- |
> | favorite_id | int  | yes  | Favorite item ID      |
>
> Response and Parameters in data
>
> no data

------

## Navigation

#### Navigation List

**nav_list**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | user_id   | int  | yes  | User ID               |
>
> Response and Parameters in data
>
> | Parameter                 | Type | Must | Parameter Description |
> | :------------------------ | :--- | :--- | :-------------------- |
> | Nav table structure (to be added later) |      |      |                       |

------

#### Add Navigation

**add_nav**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description           |
> | :-------- | :--- | :--- | :------------------------------ |
> | user_id   | int  | yes  | User ID                         |
> | type      | int  | yes  | 1/Private chat 2/Community 3/Sprite 7/Broadcast |
> | type_id   | int  | yes  | Corresponding ID                |
> | sort      | int  | yes  | Position index                  |
>
> Response and Parameters in data
>
> no data

------

#### Update Navigation

**update_nav**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | nav_id    | int  | yes  | Navigation ID         |
> | sort      | int  | no   | Position index        |
>
> Response and Parameters in data
>
> no data

------

#### Delete Navigation

**del_nav**

> Requests and Parameter Description
>
> | Parameter | Type | Must | Parameter Description |
> | :-------- | :--- | :--- | :-------------------- |
> | nav_id    | int  | yes  | Navigation ID         |
>
> Response and Parameters in data
>
> no data

------