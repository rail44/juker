# \AppsPermissionsResourcesApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_permissions_resources_list**](AppsPermissionsResourcesApi.md#apps_permissions_resources_list) | **GET** /apps.permissions.resources.list | 



## apps_permissions_resources_list

> crate::models::AppsPermissionsResourcesListSuccessSchema apps_permissions_resources_list(token, cursor, limit)


Returns list of resource grants this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**limit** | Option<**i32**> | The maximum number of items to return. |  |

### Return type

[**crate::models::AppsPermissionsResourcesListSuccessSchema**](apps_permissions_resources_list_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

