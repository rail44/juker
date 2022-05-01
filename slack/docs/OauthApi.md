# \OauthApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth_access**](OauthApi.md#oauth_access) | **GET** /oauth.access | 
[**oauth_token**](OauthApi.md#oauth_token) | **GET** /oauth.token | 
[**oauth_v2_access**](OauthApi.md#oauth_v2_access) | **GET** /oauth.v2.access | 



## oauth_access

> crate::models::DefaultSuccessTemplate oauth_access(client_id, client_secret, code, redirect_uri, single_channel)


Exchanges a temporary OAuth verifier code for an access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Issued when you created your application. |  |
**client_secret** | Option<**String**> | Issued when you created your application. |  |
**code** | Option<**String**> | The `code` param returned via the OAuth callback. |  |
**redirect_uri** | Option<**String**> | This must match the originally submitted URI (if one was sent). |  |
**single_channel** | Option<**bool**> | Request the user to add your app only to a single channel. Only valid with a [legacy workspace app](https://api.slack.com/legacy-workspace-apps). |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_token

> crate::models::DefaultSuccessTemplate oauth_token(client_id, client_secret, code, redirect_uri, single_channel)


Exchanges a temporary OAuth verifier code for a workspace token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Issued when you created your application. |  |
**client_secret** | Option<**String**> | Issued when you created your application. |  |
**code** | Option<**String**> | The `code` param returned via the OAuth callback. |  |
**redirect_uri** | Option<**String**> | This must match the originally submitted URI (if one was sent). |  |
**single_channel** | Option<**bool**> | Request the user to add your app only to a single channel. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_v2_access

> crate::models::DefaultSuccessTemplate oauth_v2_access(code, client_id, client_secret, redirect_uri)


Exchanges a temporary OAuth verifier code for an access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The `code` param returned via the OAuth callback. | [required] |
**client_id** | Option<**String**> | Issued when you created your application. |  |
**client_secret** | Option<**String**> | Issued when you created your application. |  |
**redirect_uri** | Option<**String**> | This must match the originally submitted URI (if one was sent). |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

