# \AdminTeamsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_teams_create**](AdminTeamsApi.md#admin_teams_create) | **POST** /admin.teams.create | 
[**admin_teams_list**](AdminTeamsApi.md#admin_teams_list) | **GET** /admin.teams.list | 



## admin_teams_create

> crate::models::DefaultSuccessTemplate admin_teams_create(token, team_domain, team_name, team_description, team_discoverability)


Create an Enterprise team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_domain** | **String** | Team domain (for example, slacksoftballteam). | [required] |
**team_name** | **String** | Team name (for example, Slack Softball Team). | [required] |
**team_description** | Option<**String**> | Description for the team. |  |
**team_discoverability** | Option<**String**> | Who can join the team. A team's discoverability can be `open`, `closed`, `invite_only`, or `unlisted`. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_list

> crate::models::DefaultSuccessTemplate admin_teams_list(token, limit, cursor)


List all teams on an Enterprise organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 100 both inclusive. |  |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

