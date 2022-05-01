# \UsersProfileApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_profile_get**](UsersProfileApi.md#users_profile_get) | **GET** /users.profile.get | 
[**users_profile_set**](UsersProfileApi.md#users_profile_set) | **POST** /users.profile.set | 



## users_profile_get

> crate::models::UsersProfileGetSchema users_profile_get(token, include_labels, user)


Retrieves a user's profile information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users.profile:read` | [required] |
**include_labels** | Option<**bool**> | Include labels for each ID in custom profile fields |  |
**user** | Option<**String**> | User to retrieve profile info for |  |

### Return type

[**crate::models::UsersProfileGetSchema**](users_profile_get_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_profile_set

> crate::models::UsersProfileSetSchema users_profile_set(token, name, profile, user, value)


Set the profile information for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users.profile:write` | [required] |
**name** | Option<**String**> | Name of a single key to set. Usable only if `profile` is not passed. |  |
**profile** | Option<**String**> | Collection of key:value pairs presented as a URL-encoded JSON hash. At most 50 fields may be set. Each field name is limited to 255 characters. |  |
**user** | Option<**String**> | ID of user to change. This argument may only be specified by team admins on paid teams. |  |
**value** | Option<**String**> | Value to set a single key to. Usable only if `profile` is not passed. |  |

### Return type

[**crate::models::UsersProfileSetSchema**](users_profile_set_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

