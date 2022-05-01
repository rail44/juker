# \BotsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bots_info**](BotsApi.md#bots_info) | **GET** /bots.info | 



## bots_info

> crate::models::BotsInfoSchema bots_info(token, bot)


Gets information about a bot user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:read` | [required] |
**bot** | Option<**String**> | Bot user to get info on |  |

### Return type

[**crate::models::BotsInfoSchema**](bots_info_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

