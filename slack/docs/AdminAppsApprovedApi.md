# \AdminAppsApprovedApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_apps_approved_list**](AdminAppsApprovedApi.md#admin_apps_approved_list) | **GET** /admin.apps.approved.list | 



## admin_apps_approved_list

> crate::models::DefaultSuccessTemplate admin_apps_approved_list(token, limit, cursor, team_id, enterprise_id)


List approved apps for an org or workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:read` | [required] |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**team_id** | Option<**String**> |  |  |
**enterprise_id** | Option<**String**> |  |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

