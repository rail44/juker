# \AdminAppsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_apps_approve**](AdminAppsApi.md#admin_apps_approve) | **POST** /admin.apps.approve | 
[**admin_apps_restrict**](AdminAppsApi.md#admin_apps_restrict) | **POST** /admin.apps.restrict | 



## admin_apps_approve

> crate::models::DefaultSuccessTemplate admin_apps_approve(token, app_id, request_id, team_id)


Approve an app for installation on a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:write` | [required] |
**app_id** | Option<**String**> | The id of the app to approve. |  |
**request_id** | Option<**String**> | The id of the request to approve. |  |
**team_id** | Option<**String**> |  |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_apps_restrict

> crate::models::DefaultSuccessTemplate admin_apps_restrict(token, app_id, request_id, team_id)


Restrict an app for installation on a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:write` | [required] |
**app_id** | Option<**String**> | The id of the app to restrict. |  |
**request_id** | Option<**String**> | The id of the request to restrict. |  |
**team_id** | Option<**String**> |  |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

