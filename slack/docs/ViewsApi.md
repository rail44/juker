# \ViewsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**views_open**](ViewsApi.md#views_open) | **GET** /views.open | 
[**views_publish**](ViewsApi.md#views_publish) | **GET** /views.publish | 
[**views_push**](ViewsApi.md#views_push) | **GET** /views.push | 
[**views_update**](ViewsApi.md#views_update) | **GET** /views.update | 



## views_open

> crate::models::DefaultSuccessTemplate views_open(token, trigger_id, view)


Open a view for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**trigger_id** | **String** | Exchange a trigger to post to the user. | [required] |
**view** | **String** | A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## views_publish

> crate::models::DefaultSuccessTemplate views_publish(token, user_id, view, hash)


Publish a static view for a User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**user_id** | **String** | `id` of the user you want publish a view to. | [required] |
**view** | **String** | A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | [required] |
**hash** | Option<**String**> | A string that represents view state to protect against possible race conditions. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## views_push

> crate::models::DefaultSuccessTemplate views_push(token, trigger_id, view)


Push a view onto the stack of a root view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**trigger_id** | **String** | Exchange a trigger to post to the user. | [required] |
**view** | **String** | A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## views_update

> crate::models::DefaultSuccessTemplate views_update(token, view_id, external_id, view, hash)


Update an existing view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**view_id** | Option<**String**> | A unique identifier of the view to be updated. Either `view_id` or `external_id` is required. |  |
**external_id** | Option<**String**> | A unique identifier of the view set by the developer. Must be unique for all views on a team. Max length of 255 characters. Either `view_id` or `external_id` is required. |  |
**view** | Option<**String**> | A [view object](/reference/surfaces/views). This must be a JSON-encoded string. |  |
**hash** | Option<**String**> | A string that represents view state to protect against possible race conditions. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

