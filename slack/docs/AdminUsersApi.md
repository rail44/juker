# \AdminUsersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_users_assign**](AdminUsersApi.md#admin_users_assign) | **POST** /admin.users.assign | 
[**admin_users_invite**](AdminUsersApi.md#admin_users_invite) | **POST** /admin.users.invite | 
[**admin_users_list**](AdminUsersApi.md#admin_users_list) | **GET** /admin.users.list | 
[**admin_users_remove**](AdminUsersApi.md#admin_users_remove) | **POST** /admin.users.remove | 
[**admin_users_set_admin**](AdminUsersApi.md#admin_users_set_admin) | **POST** /admin.users.setAdmin | 
[**admin_users_set_expiration**](AdminUsersApi.md#admin_users_set_expiration) | **POST** /admin.users.setExpiration | 
[**admin_users_set_owner**](AdminUsersApi.md#admin_users_set_owner) | **POST** /admin.users.setOwner | 
[**admin_users_set_regular**](AdminUsersApi.md#admin_users_set_regular) | **POST** /admin.users.setRegular | 



## admin_users_assign

> crate::models::DefaultSuccessTemplate admin_users_assign(token, team_id, user_id, is_restricted, is_ultra_restricted, channel_ids)


Add an Enterprise user to a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**user_id** | **String** | The ID of the user to add to the workspace. | [required] |
**is_restricted** | Option<**bool**> | True if user should be added to the workspace as a guest. |  |
**is_ultra_restricted** | Option<**bool**> | True if user should be added to the workspace as a single-channel guest. |  |
**channel_ids** | Option<**String**> | Comma separated values of channel IDs to add user in the new workspace. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_invite

> crate::models::DefaultSuccessTemplate admin_users_invite(token, team_id, email, channel_ids, custom_message, real_name, resend, is_restricted, is_ultra_restricted, guest_expiration_ts)


Invite a user to a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**email** | **String** | The email address of the person to invite. | [required] |
**channel_ids** | **String** | A comma-separated list of `channel_id`s for this user to join. At least one channel is required. | [required] |
**custom_message** | Option<**String**> | An optional message to send to the user in the invite email. |  |
**real_name** | Option<**String**> | Full name of the user. |  |
**resend** | Option<**bool**> | Allow this invite to be resent in the future if a user has not signed up yet. (default: false) |  |
**is_restricted** | Option<**bool**> | Is this user a multi-channel guest user? (default: false) |  |
**is_ultra_restricted** | Option<**bool**> | Is this user a single channel guest user? (default: false) |  |
**guest_expiration_ts** | Option<**String**> | Timestamp when guest account should be disabled. Only include this timestamp if you are inviting a guest user and you want their account to expire on a certain date. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_list

> crate::models::DefaultSuccessTemplate admin_users_list(token, team_id, cursor, limit)


List users on a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:read` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |
**limit** | Option<**i32**> | Limit for how many users to be retrieved per page |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_remove

> crate::models::DefaultSuccessTemplate admin_users_remove(token, team_id, user_id)


Remove a user from a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**user_id** | **String** | The ID of the user to remove. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_admin

> crate::models::DefaultSuccessTemplate admin_users_set_admin(token, team_id, user_id)


Set an existing guest, regular user, or owner to be an admin user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**user_id** | **String** | The ID of the user to designate as an admin. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_expiration

> crate::models::DefaultSuccessTemplate admin_users_set_expiration(token, team_id, user_id, expiration_ts)


Set an expiration for a guest user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**user_id** | **String** | The ID of the user to set an expiration for. | [required] |
**expiration_ts** | **i32** | Timestamp when guest account should be disabled. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_owner

> crate::models::DefaultSuccessTemplate admin_users_set_owner(token, team_id, user_id)


Set an existing guest, regular user, or admin user to be a workspace owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**user_id** | **String** | Id of the user to promote to owner. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_regular

> crate::models::DefaultSuccessTemplate admin_users_set_regular(token, team_id, user_id)


Set an existing guest user, admin user, or owner to be a regular user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**user_id** | **String** | The ID of the user to designate as a regular user. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

