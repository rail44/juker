# \TeamProfileApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**team_profile_get**](TeamProfileApi.md#team_profile_get) | **GET** /team.profile.get | 



## team_profile_get

> crate::models::TeamProfileGetSuccessSchema team_profile_get(token, visibility)


Retrieve a team's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users.profile:read` | [required] |
**visibility** | Option<**String**> | Filter by visibility. |  |

### Return type

[**crate::models::TeamProfileGetSuccessSchema**](team_profile_get_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

