# \ReactionsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reactions_add**](ReactionsApi.md#reactions_add) | **POST** /reactions.add | 
[**reactions_get**](ReactionsApi.md#reactions_get) | **GET** /reactions.get | 
[**reactions_list**](ReactionsApi.md#reactions_list) | **GET** /reactions.list | 
[**reactions_remove**](ReactionsApi.md#reactions_remove) | **POST** /reactions.remove | 



## reactions_add

> crate::models::ReactionsAddSchema reactions_add(token, channel, name, timestamp)


Adds a reaction to an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `reactions:write` | [required] |
**channel** | **String** | Channel where the message to add reaction to was posted. | [required] |
**name** | **String** | Reaction (emoji) name. | [required] |
**timestamp** | **String** | Timestamp of the message to add reaction to. | [required] |

### Return type

[**crate::models::ReactionsAddSchema**](reactions_add_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_get

> serde_json::Value reactions_get(token, channel, file, file_comment, full, timestamp)


Gets reactions for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `reactions:read` | [required] |
**channel** | Option<**String**> | Channel where the message to get reactions for was posted. |  |
**file** | Option<**String**> | File to get reactions for. |  |
**file_comment** | Option<**String**> | File comment to get reactions for. |  |
**full** | Option<**bool**> | If true always return the complete reaction list. |  |
**timestamp** | Option<**String**> | Timestamp of the message to get reactions for. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_list

> crate::models::ReactionsListSchema reactions_list(token, user, full, count, page, cursor, limit)


Lists reactions made by a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `reactions:read` | [required] |
**user** | Option<**String**> | Show reactions made by this user. Defaults to the authed user. |  |
**full** | Option<**bool**> | If true always return the complete reaction list. |  |
**count** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |
**cursor** | Option<**String**> | Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more details. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |

### Return type

[**crate::models::ReactionsListSchema**](reactions_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_remove

> crate::models::ReactionsRemoveSchema reactions_remove(token, name, file, file_comment, channel, timestamp)


Removes a reaction from an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `reactions:write` | [required] |
**name** | **String** | Reaction (emoji) name. | [required] |
**file** | Option<**String**> | File to remove reaction from. |  |
**file_comment** | Option<**String**> | File comment to remove reaction from. |  |
**channel** | Option<**String**> | Channel where the message to remove reaction from was posted. |  |
**timestamp** | Option<**String**> | Timestamp of the message to remove reaction from. |  |

### Return type

[**crate::models::ReactionsRemoveSchema**](reactions_remove_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

