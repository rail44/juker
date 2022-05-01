# \AppsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_event_authorizations_list**](AppsApi.md#apps_event_authorizations_list) | **GET** /apps.event.authorizations.list | 
[**apps_permissions_info**](AppsApi.md#apps_permissions_info) | **GET** /apps.permissions.info | 
[**apps_permissions_request**](AppsApi.md#apps_permissions_request) | **GET** /apps.permissions.request | 
[**apps_permissions_resources_list**](AppsApi.md#apps_permissions_resources_list) | **GET** /apps.permissions.resources.list | 
[**apps_permissions_scopes_list**](AppsApi.md#apps_permissions_scopes_list) | **GET** /apps.permissions.scopes.list | 
[**apps_permissions_users_list**](AppsApi.md#apps_permissions_users_list) | **GET** /apps.permissions.users.list | 
[**apps_permissions_users_request**](AppsApi.md#apps_permissions_users_request) | **GET** /apps.permissions.users.request | 
[**apps_uninstall**](AppsApi.md#apps_uninstall) | **GET** /apps.uninstall | 



## apps_event_authorizations_list

> crate::models::DefaultSuccessTemplate apps_event_authorizations_list(token, event_context, cursor, limit)


Get a list of authorizations for the given event context. Each authorization represents an app installation that the event is visible to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `authorizations:read` | [required] |
**event_context** | **String** |  | [required] |
**cursor** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## apps_permissions_users_list

> crate::models::DefaultSuccessTemplate apps_permissions_users_list(token, cursor, limit)


Returns list of user grants and corresponding scopes this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**limit** | Option<**i32**> | The maximum number of items to return. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_permissions_users_request

> crate::models::DefaultSuccessTemplate apps_permissions_users_request(token, scopes, trigger_id, user)


Enables an app to trigger a permissions modal to grant an app access to a user access scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**scopes** | **String** | A comma separated list of user scopes to request for | [required] |
**trigger_id** | **String** | Token used to trigger the request | [required] |
**user** | **String** | The user this scope is being requested for | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_uninstall

> crate::models::AppsUninstallSchema apps_uninstall(token, client_id, client_secret)


Uninstalls your app from a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |
**client_id** | Option<**String**> | Issued when you created your application. |  |
**client_secret** | Option<**String**> | Issued when you created your application. |  |

### Return type

[**crate::models::AppsUninstallSchema**](apps_uninstall_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

