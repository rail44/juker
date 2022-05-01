# \AdminUsergroupsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_usergroups_add_channels**](AdminUsergroupsApi.md#admin_usergroups_add_channels) | **POST** /admin.usergroups.addChannels | 
[**admin_usergroups_add_teams**](AdminUsergroupsApi.md#admin_usergroups_add_teams) | **POST** /admin.usergroups.addTeams | 
[**admin_usergroups_list_channels**](AdminUsergroupsApi.md#admin_usergroups_list_channels) | **GET** /admin.usergroups.listChannels | 
[**admin_usergroups_remove_channels**](AdminUsergroupsApi.md#admin_usergroups_remove_channels) | **POST** /admin.usergroups.removeChannels | 



## admin_usergroups_add_channels

> crate::models::DefaultSuccessTemplate admin_usergroups_add_channels(token, usergroup_id, channel_ids, team_id)


Add one or more default channels to an IDP group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.usergroups:write` | [required] |
**usergroup_id** | **String** | ID of the IDP group to add default channels for. | [required] |
**channel_ids** | **String** | Comma separated string of channel IDs. | [required] |
**team_id** | Option<**String**> | The workspace to add default channels in. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_usergroups_add_teams

> crate::models::DefaultSuccessTemplate admin_usergroups_add_teams(token, usergroup_id, team_ids, auto_provision)


Associate one or more default workspaces with an organization-wide IDP group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**usergroup_id** | **String** | An encoded usergroup (IDP Group) ID. | [required] |
**team_ids** | **String** | A comma separated list of encoded team (workspace) IDs. Each workspace *MUST* belong to the organization associated with the token. | [required] |
**auto_provision** | Option<**bool**> | When `true`, this method automatically creates new workspace accounts for the IDP group members. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_usergroups_list_channels

> crate::models::DefaultSuccessTemplate admin_usergroups_list_channels(token, usergroup_id, team_id, include_num_members)


List the channels linked to an org-level IDP group (user group).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.usergroups:read` | [required] |
**usergroup_id** | **String** | ID of the IDP group to list default channels for. | [required] |
**team_id** | Option<**String**> | ID of the the workspace. |  |
**include_num_members** | Option<**bool**> | Flag to include or exclude the count of members per channel. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_usergroups_remove_channels

> crate::models::DefaultSuccessTemplate admin_usergroups_remove_channels(token, usergroup_id, channel_ids)


Remove one or more default channels from an org-level IDP group (user group).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.usergroups:write` | [required] |
**usergroup_id** | **String** | ID of the IDP Group | [required] |
**channel_ids** | **String** | Comma-separated string of channel IDs | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

