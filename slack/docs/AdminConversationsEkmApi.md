# \AdminConversationsEkmApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_conversations_ekm_list_original_connected_channel_info**](AdminConversationsEkmApi.md#admin_conversations_ekm_list_original_connected_channel_info) | **GET** /admin.conversations.ekm.listOriginalConnectedChannelInfo | 



## admin_conversations_ekm_list_original_connected_channel_info

> crate::models::DefaultSuccessTemplate admin_conversations_ekm_list_original_connected_channel_info(token, channel_ids, team_ids, limit, cursor)


List all disconnected channels—i.e., channels that were once connected to other workspaces and then disconnected—and the corresponding original channel IDs for key revocation with EKM.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:read` | [required] |
**channel_ids** | Option<**String**> | A comma-separated list of channels to filter to. |  |
**team_ids** | Option<**String**> | A comma-separated list of the workspaces to which the channels you would like returned belong. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

