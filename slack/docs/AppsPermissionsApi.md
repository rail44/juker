# \AppsPermissionsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_permissions_info**](AppsPermissionsApi.md#apps_permissions_info) | **GET** /apps.permissions.info | 
[**apps_permissions_request**](AppsPermissionsApi.md#apps_permissions_request) | **GET** /apps.permissions.request | 



## apps_permissions_info

> crate::models::AppsPermissionsInfoSchema apps_permissions_info(token)


Returns list of permissions this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |

### Return type

[**crate::models::AppsPermissionsInfoSchema**](apps_permissions_info_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_permissions_request

> crate::models::AppsPermissionsRequestSchema apps_permissions_request(token, scopes, trigger_id)


Allows an app to request additional scopes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**scopes** | **String** | A comma separated list of scopes to request for | [required] |
**trigger_id** | **String** | Token used to trigger the permissions API | [required] |

### Return type

[**crate::models::AppsPermissionsRequestSchema**](apps_permissions_request_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

