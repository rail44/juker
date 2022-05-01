# \SearchApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_messages**](SearchApi.md#search_messages) | **GET** /search.messages | 



## search_messages

> crate::models::DefaultSuccessTemplate search_messages(token, query, count, highlight, page, sort, sort_dir)


Searches for messages matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `search:read` | [required] |
**query** | **String** | Search query. | [required] |
**count** | Option<**i32**> | Pass the number of results you want per \"page\". Maximum of `100`. |  |
**highlight** | Option<**bool**> | Pass a value of `true` to enable query highlight markers (see below). |  |
**page** | Option<**i32**> |  |  |
**sort** | Option<**String**> | Return matches sorted by either `score` or `timestamp`. |  |
**sort_dir** | Option<**String**> | Change sort direction to ascending (`asc`) or descending (`desc`). |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

