# \AdminConversationsRestrictAccessApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_conversations_restrict_access_add_group**](AdminConversationsRestrictAccessApi.md#admin_conversations_restrict_access_add_group) | **POST** /admin.conversations.restrictAccess.addGroup | 
[**admin_conversations_restrict_access_list_groups**](AdminConversationsRestrictAccessApi.md#admin_conversations_restrict_access_list_groups) | **GET** /admin.conversations.restrictAccess.listGroups | 
[**admin_conversations_restrict_access_remove_group**](AdminConversationsRestrictAccessApi.md#admin_conversations_restrict_access_remove_group) | **POST** /admin.conversations.restrictAccess.removeGroup | 



## admin_conversations_restrict_access_add_group

> crate::models::DefaultSuccessTemplate admin_conversations_restrict_access_add_group(token, group_id, channel_id, team_id)


Add an allowlist of IDP groups for accessing a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**group_id** | **String** | The [IDP Group](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-Grid-org) ID to be an allowlist for the private channel. | [required] |
**channel_id** | **String** | The channel to link this group to. | [required] |
**team_id** | Option<**String**> | The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_restrict_access_list_groups

> crate::models::DefaultSuccessTemplate admin_conversations_restrict_access_list_groups(token, channel_id, team_id)


List all IDP Groups linked to a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:read` | [required] |
**channel_id** | **String** |  | [required] |
**team_id** | Option<**String**> | The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_restrict_access_remove_group

> crate::models::DefaultSuccessTemplate admin_conversations_restrict_access_remove_group(token, team_id, group_id, channel_id)


Remove a linked IDP group linked from a private channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**team_id** | **String** | The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization. | [required] |
**group_id** | **String** | The [IDP Group](https://slack.com/help/articles/115001435788-Connect-identity-provider-groups-to-your-Enterprise-Grid-org) ID to remove from the private channel. | [required] |
**channel_id** | **String** | The channel to remove the linked group from. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

