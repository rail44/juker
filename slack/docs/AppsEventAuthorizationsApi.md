# \AppsEventAuthorizationsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_event_authorizations_list**](AppsEventAuthorizationsApi.md#apps_event_authorizations_list) | **GET** /apps.event.authorizations.list | 



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

