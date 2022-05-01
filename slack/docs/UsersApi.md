# \UsersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_conversations**](UsersApi.md#users_conversations) | **GET** /users.conversations | 
[**users_delete_photo**](UsersApi.md#users_delete_photo) | **POST** /users.deletePhoto | 
[**users_get_presence**](UsersApi.md#users_get_presence) | **GET** /users.getPresence | 
[**users_identity**](UsersApi.md#users_identity) | **GET** /users.identity | 
[**users_info**](UsersApi.md#users_info) | **GET** /users.info | 
[**users_list**](UsersApi.md#users_list) | **GET** /users.list | 
[**users_lookup_by_email**](UsersApi.md#users_lookup_by_email) | **GET** /users.lookupByEmail | 
[**users_profile_get**](UsersApi.md#users_profile_get) | **GET** /users.profile.get | 
[**users_profile_set**](UsersApi.md#users_profile_set) | **POST** /users.profile.set | 
[**users_set_active**](UsersApi.md#users_set_active) | **POST** /users.setActive | 
[**users_set_photo**](UsersApi.md#users_set_photo) | **POST** /users.setPhoto | 
[**users_set_presence**](UsersApi.md#users_set_presence) | **POST** /users.setPresence | 



## users_conversations

> crate::models::UsersConversationsSuccessSchema users_conversations(token, user, types, exclude_archived, limit, cursor)


List conversations the calling user may access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**user** | Option<**String**> | Browse conversations by a specific user ID's membership. Non-public channels are restricted to those where the calling user shares membership. |  |
**types** | Option<**String**> | Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im` |  |
**exclude_archived** | Option<**bool**> | Set to `true` to exclude archived channels from the list |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**crate::models::UsersConversationsSuccessSchema**](users_conversations_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_delete_photo

> crate::models::UsersDeletePhotoSchema users_delete_photo(token)


Delete the user profile photo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users.profile:write` | [required] |

### Return type

[**crate::models::UsersDeletePhotoSchema**](users_deletePhoto_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get_presence

> crate::models::ApiMethodUsersGetPresence users_get_presence(token, user)


Gets user presence information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:read` | [required] |
**user** | Option<**String**> | User to get presence info on. Defaults to the authed user. |  |

### Return type

[**crate::models::ApiMethodUsersGetPresence**](API_method__users_getPresence.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_identity

> serde_json::Value users_identity(token)


Get a user's identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `identity.basic` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_info

> crate::models::UsersInfoSuccessSchema users_info(token, include_locale, user)


Gets information about a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:read` | [required] |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this user. Defaults to `false` |  |
**user** | Option<**String**> | User to get info on |  |

### Return type

[**crate::models::UsersInfoSuccessSchema**](users_info_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list

> crate::models::UsersListSchema users_list(token, limit, cursor, include_locale)


Lists all users in a Slack team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `users:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. Providing no `limit` value will result in Slack attempting to deliver you the entire result set. If the collection is too large you may experience `limit_required` or HTTP 500 errors. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for users. Defaults to `false` |  |

### Return type

[**crate::models::UsersListSchema**](users_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_lookup_by_email

> crate::models::UsersLookupByEmailSuccessSchema users_lookup_by_email(token, email)


Find a user with an email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:read.email` | [required] |
**email** | **String** | An email address belonging to a user in the workspace | [required] |

### Return type

[**crate::models::UsersLookupByEmailSuccessSchema**](users_lookupByEmail_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## users_set_active

> crate::models::UsersSetActiveSchema users_set_active(token)


Marked a user as active. Deprecated and non-functional.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:write` | [required] |

### Return type

[**crate::models::UsersSetActiveSchema**](users_setActive_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_set_photo

> crate::models::UsersSetPhotoSchema users_set_photo(token, crop_w, crop_x, crop_y, image)


Set the user profile photo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users.profile:write` | [required] |
**crop_w** | Option<**String**> | Width/height of crop box (always square) |  |
**crop_x** | Option<**String**> | X coordinate of top-left corner of crop box |  |
**crop_y** | Option<**String**> | Y coordinate of top-left corner of crop box |  |
**image** | Option<**String**> | File contents via `multipart/form-data`. |  |

### Return type

[**crate::models::UsersSetPhotoSchema**](users_setPhoto_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_set_presence

> crate::models::UsersSetPresenceSchema users_set_presence(token, presence)


Manually sets user presence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:write` | [required] |
**presence** | **String** | Either `auto` or `away` | [required] |

### Return type

[**crate::models::UsersSetPresenceSchema**](users_setPresence_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

