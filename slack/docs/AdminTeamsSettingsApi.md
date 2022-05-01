# \AdminTeamsSettingsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_teams_settings_info**](AdminTeamsSettingsApi.md#admin_teams_settings_info) | **GET** /admin.teams.settings.info | 
[**admin_teams_settings_set_default_channels**](AdminTeamsSettingsApi.md#admin_teams_settings_set_default_channels) | **POST** /admin.teams.settings.setDefaultChannels | 
[**admin_teams_settings_set_description**](AdminTeamsSettingsApi.md#admin_teams_settings_set_description) | **POST** /admin.teams.settings.setDescription | 
[**admin_teams_settings_set_discoverability**](AdminTeamsSettingsApi.md#admin_teams_settings_set_discoverability) | **POST** /admin.teams.settings.setDiscoverability | 
[**admin_teams_settings_set_icon**](AdminTeamsSettingsApi.md#admin_teams_settings_set_icon) | **POST** /admin.teams.settings.setIcon | 
[**admin_teams_settings_set_name**](AdminTeamsSettingsApi.md#admin_teams_settings_set_name) | **POST** /admin.teams.settings.setName | 



## admin_teams_settings_info

> crate::models::DefaultSuccessTemplate admin_teams_settings_info(token, team_id)


Fetch information about settings in a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**team_id** | **String** |  | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_default_channels

> crate::models::DefaultSuccessTemplate admin_teams_settings_set_default_channels(token, team_id, channel_ids)


Set the default channels of a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_id** | **String** | ID for the workspace to set the default channel for. | [required] |
**channel_ids** | **String** | An array of channel IDs. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_description

> crate::models::DefaultSuccessTemplate admin_teams_settings_set_description(token, team_id, description)


Set the description of a given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_id** | **String** | ID for the workspace to set the description for. | [required] |
**description** | **String** | The new description for the workspace. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_discoverability

> crate::models::DefaultSuccessTemplate admin_teams_settings_set_discoverability(token, team_id, discoverability)


An API method that allows admins to set the discoverability of a given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_id** | **String** | The ID of the workspace to set discoverability on. | [required] |
**discoverability** | **String** | This workspace's discovery setting. It must be set to one of `open`, `invite_only`, `closed`, or `unlisted`. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_icon

> crate::models::DefaultSuccessTemplate admin_teams_settings_set_icon(token, image_url, team_id)


Sets the icon of a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**image_url** | **String** | Image URL for the icon | [required] |
**team_id** | **String** | ID for the workspace to set the icon for. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_name

> crate::models::DefaultSuccessTemplate admin_teams_settings_set_name(token, team_id, name)


Set the name of a given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_id** | **String** | ID for the workspace to set the name for. | [required] |
**name** | **String** | The new name of the workspace. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

