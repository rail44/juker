# \ConversationsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**conversations_archive**](ConversationsApi.md#conversations_archive) | **POST** /conversations.archive | 
[**conversations_close**](ConversationsApi.md#conversations_close) | **POST** /conversations.close | 
[**conversations_create**](ConversationsApi.md#conversations_create) | **POST** /conversations.create | 
[**conversations_history**](ConversationsApi.md#conversations_history) | **GET** /conversations.history | 
[**conversations_info**](ConversationsApi.md#conversations_info) | **GET** /conversations.info | 
[**conversations_invite**](ConversationsApi.md#conversations_invite) | **POST** /conversations.invite | 
[**conversations_join**](ConversationsApi.md#conversations_join) | **POST** /conversations.join | 
[**conversations_kick**](ConversationsApi.md#conversations_kick) | **POST** /conversations.kick | 
[**conversations_leave**](ConversationsApi.md#conversations_leave) | **POST** /conversations.leave | 
[**conversations_list**](ConversationsApi.md#conversations_list) | **GET** /conversations.list | 
[**conversations_mark**](ConversationsApi.md#conversations_mark) | **POST** /conversations.mark | 
[**conversations_members**](ConversationsApi.md#conversations_members) | **GET** /conversations.members | 
[**conversations_open**](ConversationsApi.md#conversations_open) | **POST** /conversations.open | 
[**conversations_rename**](ConversationsApi.md#conversations_rename) | **POST** /conversations.rename | 
[**conversations_replies**](ConversationsApi.md#conversations_replies) | **GET** /conversations.replies | 
[**conversations_set_purpose**](ConversationsApi.md#conversations_set_purpose) | **POST** /conversations.setPurpose | 
[**conversations_set_topic**](ConversationsApi.md#conversations_set_topic) | **POST** /conversations.setTopic | 
[**conversations_unarchive**](ConversationsApi.md#conversations_unarchive) | **POST** /conversations.unarchive | 



## conversations_archive

> crate::models::ConversationsArchiveSuccessSchema conversations_archive(token, channel)


Archives a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to archive |  |

### Return type

[**crate::models::ConversationsArchiveSuccessSchema**](conversations_archive_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_close

> crate::models::ConversationsCloseSuccessSchema conversations_close(token, channel)


Closes a direct message or multi-person direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to close. |  |

### Return type

[**crate::models::ConversationsCloseSuccessSchema**](conversations_close_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_create

> crate::models::ConversationsCreateSuccessSchema conversations_create(token, name, is_private)


Initiates a public or private channel-based conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**name** | Option<**String**> | Name of the public or private channel to create |  |
**is_private** | Option<**bool**> | Create a private channel instead of a public one |  |

### Return type

[**crate::models::ConversationsCreateSuccessSchema**](conversations_create_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_history

> crate::models::ConversationsHistorySuccessSchema conversations_history(token, channel, latest, oldest, inclusive, limit, cursor)


Fetches a conversation's history of messages and events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:history` |  |
**channel** | Option<**String**> | Conversation ID to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results only when either timestamp is specified. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**crate::models::ConversationsHistorySuccessSchema**](conversations_history_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_info

> crate::models::ConversationsInfoSuccessSchema conversations_info(token, channel, include_locale, include_num_members)


Retrieve information about a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**channel** | Option<**String**> | Conversation ID to learn more about |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this conversation. Defaults to `false` |  |
**include_num_members** | Option<**bool**> | Set to `true` to include the member count for the specified conversation. Defaults to `false` |  |

### Return type

[**crate::models::ConversationsInfoSuccessSchema**](conversations_info_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_invite

> crate::models::ConversationsInviteErrorSchema conversations_invite(token, channel, users)


Invites users to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | The ID of the public or private channel to invite user(s) to. |  |
**users** | Option<**String**> | A comma separated list of user IDs. Up to 1000 users may be listed. |  |

### Return type

[**crate::models::ConversationsInviteErrorSchema**](conversations_invite_error_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_join

> crate::models::ConversationsJoinSuccessSchema conversations_join(token, channel)


Joins an existing conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**channel** | Option<**String**> | ID of conversation to join |  |

### Return type

[**crate::models::ConversationsJoinSuccessSchema**](conversations_join_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_kick

> crate::models::ConversationsKickSuccessSchema conversations_kick(token, channel, user)


Removes a user from a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to remove user from. |  |
**user** | Option<**String**> | User ID to be removed. |  |

### Return type

[**crate::models::ConversationsKickSuccessSchema**](conversations_kick_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_leave

> crate::models::ConversationsLeaveSuccessSchema conversations_leave(token, channel)


Leaves a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to leave |  |

### Return type

[**crate::models::ConversationsLeaveSuccessSchema**](conversations_leave_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_list

> crate::models::ConversationsListSuccessSchema conversations_list(token, exclude_archived, types, limit, cursor)


Lists all channels in a Slack team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**exclude_archived** | Option<**bool**> | Set to `true` to exclude archived channels from the list |  |
**types** | Option<**String**> | Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**crate::models::ConversationsListSuccessSchema**](conversations_list_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_mark

> crate::models::ConversationsMarkSuccessSchema conversations_mark(token, channel, ts)


Sets the read cursor in a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Channel or conversation to set the read cursor for. |  |
**ts** | Option<**f32**> | Unique identifier of message you want marked as most recently seen in this conversation. |  |

### Return type

[**crate::models::ConversationsMarkSuccessSchema**](conversations_mark_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_members

> crate::models::ConversationsMembersSuccessSchema conversations_members(token, channel, limit, cursor)


Retrieve members of a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**channel** | Option<**String**> | ID of the conversation to retrieve members for |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**crate::models::ConversationsMembersSuccessSchema**](conversations_members_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_open

> crate::models::ConversationsOpenSuccessSchema conversations_open(token, channel, users, return_im)


Opens or resumes a direct message or multi-person direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Resume a conversation by supplying an `im` or `mpim`'s ID. Or provide the `users` field instead. |  |
**users** | Option<**String**> | Comma separated lists of users. If only one user is included, this creates a 1:1 DM.  The ordering of the users is preserved whenever a multi-person direct message is returned. Supply a `channel` when not supplying `users`. |  |
**return_im** | Option<**bool**> | Boolean, indicates you want the full IM channel definition in the response. |  |

### Return type

[**crate::models::ConversationsOpenSuccessSchema**](conversations_open_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_rename

> crate::models::ConversationsRenameSuccessSchema conversations_rename(token, channel, name)


Renames a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to rename |  |
**name** | Option<**String**> | New name for conversation. |  |

### Return type

[**crate::models::ConversationsRenameSuccessSchema**](conversations_rename_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_replies

> crate::models::ConversationsRepliesSuccessSchema conversations_replies(token, channel, ts, latest, oldest, inclusive, limit, cursor)


Retrieve a thread of messages posted to a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:history` |  |
**channel** | Option<**String**> | Conversation ID to fetch thread from. |  |
**ts** | Option<**f32**> | Unique identifier of a thread's parent message. `ts` must be the timestamp of an existing message with 0 or more replies. If there are no replies then just the single message referenced by `ts` will return - it is just an ordinary, unthreaded message. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results only when either timestamp is specified. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**crate::models::ConversationsRepliesSuccessSchema**](conversations_replies_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_set_purpose

> crate::models::ConversationsSetPurposeSuccessSchema conversations_set_purpose(token, channel, purpose)


Sets the purpose for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to set the purpose of |  |
**purpose** | Option<**String**> | A new, specialer purpose |  |

### Return type

[**crate::models::ConversationsSetPurposeSuccessSchema**](conversations_setPurpose_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_set_topic

> crate::models::ConversationsSetTopicSuccessSchema conversations_set_topic(token, channel, topic)


Sets the topic for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to set the topic of |  |
**topic** | Option<**String**> | The new topic string. Does not support formatting or linkification. |  |

### Return type

[**crate::models::ConversationsSetTopicSuccessSchema**](conversations_setTopic_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_unarchive

> crate::models::ConversationsUnarchiveSuccessSchema conversations_unarchive(token, channel)


Reverses conversation archival.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to unarchive |  |

### Return type

[**crate::models::ConversationsUnarchiveSuccessSchema**](conversations_unarchive_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

