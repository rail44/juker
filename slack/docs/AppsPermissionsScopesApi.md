# \AppsPermissionsScopesApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_permissions_scopes_list**](AppsPermissionsScopesApi.md#apps_permissions_scopes_list) | **GET** /apps.permissions.scopes.list | 



## apps_permissions_scopes_list

> crate::models::ApiPermissionsScopesListSuccessSchema apps_permissions_scopes_list(token)


Returns list of scopes this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |

### Return type

[**crate::models::ApiPermissionsScopesListSuccessSchema**](api_permissions_scopes_list_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

