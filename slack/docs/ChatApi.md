# \ChatApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_delete**](ChatApi.md#chat_delete) | **POST** /chat.delete | 
[**chat_delete_scheduled_message**](ChatApi.md#chat_delete_scheduled_message) | **POST** /chat.deleteScheduledMessage | 
[**chat_get_permalink**](ChatApi.md#chat_get_permalink) | **GET** /chat.getPermalink | 
[**chat_me_message**](ChatApi.md#chat_me_message) | **POST** /chat.meMessage | 
[**chat_post_ephemeral**](ChatApi.md#chat_post_ephemeral) | **POST** /chat.postEphemeral | 
[**chat_post_message**](ChatApi.md#chat_post_message) | **POST** /chat.postMessage | 
[**chat_schedule_message**](ChatApi.md#chat_schedule_message) | **POST** /chat.scheduleMessage | 
[**chat_scheduled_messages_list**](ChatApi.md#chat_scheduled_messages_list) | **GET** /chat.scheduledMessages.list | 
[**chat_unfurl**](ChatApi.md#chat_unfurl) | **POST** /chat.unfurl | 
[**chat_update**](ChatApi.md#chat_update) | **POST** /chat.update | 



## chat_delete

> crate::models::ChatDeleteSuccessSchema chat_delete(token, ts, channel, as_user)


Deletes a message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `chat:write` |  |
**ts** | Option<**f32**> | Timestamp of the message to be deleted. |  |
**channel** | Option<**String**> | Channel containing the message to be deleted. |  |
**as_user** | Option<**bool**> | Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope. |  |

### Return type

[**crate::models::ChatDeleteSuccessSchema**](chat_delete_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_delete_scheduled_message

> crate::models::ChatDeleteScheduledMessageSchema chat_delete_scheduled_message(token, channel, scheduled_message_id, as_user)


Deletes a pending scheduled message from the queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `chat:write` | [required] |
**channel** | **String** | The channel the scheduled_message is posting to | [required] |
**scheduled_message_id** | **String** | `scheduled_message_id` returned from call to chat.scheduleMessage | [required] |
**as_user** | Option<**bool**> | Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope. |  |

### Return type

[**crate::models::ChatDeleteScheduledMessageSchema**](chat_deleteScheduledMessage_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_get_permalink

> crate::models::ChatGetPermalinkSuccessSchema chat_get_permalink(token, channel, message_ts)


Retrieve a permalink URL for a specific extant message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**channel** | **String** | The ID of the conversation or channel containing the message | [required] |
**message_ts** | **String** | A message's `ts` value, uniquely identifying it within a channel | [required] |

### Return type

[**crate::models::ChatGetPermalinkSuccessSchema**](chat_getPermalink_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_me_message

> crate::models::ChatMeMessageSchema chat_me_message(token, channel, text)


Share a me message into a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `chat:write` |  |
**channel** | Option<**String**> | Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name. |  |
**text** | Option<**String**> | Text of the message to send. |  |

### Return type

[**crate::models::ChatMeMessageSchema**](chat_meMessage_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_post_ephemeral

> crate::models::ChatPostEphemeralSuccessSchema chat_post_ephemeral(token, channel, user, as_user, attachments, blocks, icon_emoji, icon_url, link_names, parse, text, thread_ts, username)


Sends an ephemeral message to a user in a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `chat:write` | [required] |
**channel** | **String** | Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. | [required] |
**user** | **String** | `id` of the user who will receive the ephemeral message. The user should be in the channel specified by the `channel` argument. | [required] |
**as_user** | Option<**bool**> | Pass true to post the message as the authed user. Defaults to true if the chat:write:bot scope is not included. Otherwise, defaults to false. |  |
**attachments** | Option<**String**> | A JSON-based array of structured attachments, presented as a URL-encoded string. |  |
**blocks** | Option<**String**> | A JSON-based array of structured blocks, presented as a URL-encoded string. |  |
**icon_emoji** | Option<**String**> | Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below. |  |
**icon_url** | Option<**String**> | URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below. |  |
**link_names** | Option<**bool**> | Find and link channel names and usernames. |  |
**parse** | Option<**String**> | Change how messages are treated. Defaults to `none`. See [below](#formatting). |  |
**text** | Option<**String**> | How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. |  |
**thread_ts** | Option<**String**> | Provide another message's `ts` value to post this message in a thread. Avoid using a reply's `ts` value; use its parent's value instead. Ephemeral messages in threads are only shown if there is already an active thread. |  |
**username** | Option<**String**> | Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below. |  |

### Return type

[**crate::models::ChatPostEphemeralSuccessSchema**](chat_postEphemeral_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_post_message

> crate::models::ChatPostMessageSuccessSchema chat_post_message(token, channel, as_user, attachments, blocks, icon_emoji, icon_url, link_names, mrkdwn, parse, reply_broadcast, text, thread_ts, unfurl_links, unfurl_media, username)


Sends a message to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `chat:write` | [required] |
**channel** | **String** | Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details. | [required] |
**as_user** | Option<**String**> | Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [authorship](#authorship) below. |  |
**attachments** | Option<**String**> | A JSON-based array of structured attachments, presented as a URL-encoded string. |  |
**blocks** | Option<**String**> | A JSON-based array of structured blocks, presented as a URL-encoded string. |  |
**icon_emoji** | Option<**String**> | Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below. |  |
**icon_url** | Option<**String**> | URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below. |  |
**link_names** | Option<**bool**> | Find and link channel names and usernames. |  |
**mrkdwn** | Option<**bool**> | Disable Slack markup parsing by setting to `false`. Enabled by default. |  |
**parse** | Option<**String**> | Change how messages are treated. Defaults to `none`. See [below](#formatting). |  |
**reply_broadcast** | Option<**bool**> | Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`. |  |
**text** | Option<**String**> | How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. |  |
**thread_ts** | Option<**String**> | Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead. |  |
**unfurl_links** | Option<**bool**> | Pass true to enable unfurling of primarily text-based content. |  |
**unfurl_media** | Option<**bool**> | Pass false to disable unfurling of media content. |  |
**username** | Option<**String**> | Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below. |  |

### Return type

[**crate::models::ChatPostMessageSuccessSchema**](chat_postMessage_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_schedule_message

> crate::models::ChatScheduleMessageSuccessSchema chat_schedule_message(token, channel, text, post_at, parse, as_user, link_names, attachments, blocks, unfurl_links, unfurl_media, thread_ts, reply_broadcast)


Schedules a message to be sent to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `chat:write` |  |
**channel** | Option<**String**> | Channel, private group, or DM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details. |  |
**text** | Option<**String**> | How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. |  |
**post_at** | Option<**String**> | Unix EPOCH timestamp of time in future to send the message. |  |
**parse** | Option<**String**> | Change how messages are treated. Defaults to `none`. See [chat.postMessage](chat.postMessage#formatting). |  |
**as_user** | Option<**bool**> | Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [chat.postMessage](chat.postMessage#authorship). |  |
**link_names** | Option<**bool**> | Find and link channel names and usernames. |  |
**attachments** | Option<**String**> | A JSON-based array of structured attachments, presented as a URL-encoded string. |  |
**blocks** | Option<**String**> | A JSON-based array of structured blocks, presented as a URL-encoded string. |  |
**unfurl_links** | Option<**bool**> | Pass true to enable unfurling of primarily text-based content. |  |
**unfurl_media** | Option<**bool**> | Pass false to disable unfurling of media content. |  |
**thread_ts** | Option<**f32**> | Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead. |  |
**reply_broadcast** | Option<**bool**> | Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`. |  |

### Return type

[**crate::models::ChatScheduleMessageSuccessSchema**](chat_scheduleMessage_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_scheduled_messages_list

> crate::models::ChatScheduledMessagesListSchema chat_scheduled_messages_list(token, channel, latest, oldest, limit, cursor)


Returns a list of scheduled messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |
**channel** | Option<**String**> | The channel of the scheduled messages |  |
**latest** | Option<**f32**> | A UNIX timestamp of the latest value in the time range |  |
**oldest** | Option<**f32**> | A UNIX timestamp of the oldest value in the time range |  |
**limit** | Option<**i32**> | Maximum number of original entries to return. |  |
**cursor** | Option<**String**> | For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from. |  |

### Return type

[**crate::models::ChatScheduledMessagesListSchema**](chat_scheduledMessages_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_unfurl

> crate::models::ChatUnfurlSuccessSchema chat_unfurl(token, channel, ts, unfurls, user_auth_message, user_auth_required, user_auth_url)


Provide custom unfurl behavior for user-posted URLs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `links:write` | [required] |
**channel** | **String** | Channel ID of the message | [required] |
**ts** | **String** | Timestamp of the message to add unfurl behavior to. | [required] |
**unfurls** | Option<**String**> | URL-encoded JSON map with keys set to URLs featured in the the message, pointing to their unfurl blocks or message attachments. |  |
**user_auth_message** | Option<**String**> | Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior |  |
**user_auth_required** | Option<**bool**> | Set to `true` or `1` to indicate the user must install your Slack app to trigger unfurls for this domain |  |
**user_auth_url** | Option<**String**> | Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded. |  |

### Return type

[**crate::models::ChatUnfurlSuccessSchema**](chat_unfurl_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_update

> crate::models::ChatUpdateSuccessSchema chat_update(token, channel, ts, as_user, attachments, blocks, link_names, parse, text)


Updates a message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `chat:write` | [required] |
**channel** | **String** | Channel containing the message to be updated. | [required] |
**ts** | **String** | Timestamp of the message to be updated. | [required] |
**as_user** | Option<**String**> | Pass true to update the message as the authed user. [Bot users](/bot-users) in this context are considered authed users. |  |
**attachments** | Option<**String**> | A JSON-based array of structured attachments, presented as a URL-encoded string. This field is required when not presenting `text`. If you don't include this field, the message's previous `attachments` will be retained. To remove previous `attachments`, include an empty array for this field. |  |
**blocks** | Option<**String**> | A JSON-based array of [structured blocks](/block-kit/building), presented as a URL-encoded string. If you don't include this field, the message's previous `blocks` will be retained. To remove previous `blocks`, include an empty array for this field. |  |
**link_names** | Option<**String**> | Find and link channel names and usernames. Defaults to `none`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `none`. |  |
**parse** | Option<**String**> | Change how messages are treated. Defaults to `client`, unlike `chat.postMessage`. Accepts either `none` or `full`. If you do not specify a value for this field, the original value set for the message will be overwritten with the default, `client`. |  |
**text** | Option<**String**> | New text for the message, using the [default formatting rules](/reference/surfaces/formatting). It's not required when presenting `blocks` or `attachments`. |  |

### Return type

[**crate::models::ChatUpdateSuccessSchema**](chat_update_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

