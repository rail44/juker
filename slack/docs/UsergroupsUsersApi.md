# \UsergroupsUsersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**usergroups_users_list**](UsergroupsUsersApi.md#usergroups_users_list) | **GET** /usergroups.users.list | 
[**usergroups_users_update**](UsergroupsUsersApi.md#usergroups_users_update) | **POST** /usergroups.users.update | 



## usergroups_users_list

> crate::models::UsergroupsUsersListSchema usergroups_users_list(token, usergroup, include_disabled)


List all users in a User Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:read` | [required] |
**usergroup** | **String** | The encoded ID of the User Group to update. | [required] |
**include_disabled** | Option<**bool**> | Allow results that involve disabled User Groups. |  |

### Return type

[**crate::models::UsergroupsUsersListSchema**](usergroups_users_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usergroups_users_update

> crate::models::UsergroupsUsersUpdateSchema usergroups_users_update(token, usergroup, users, include_count)


Update the list of users for a User Group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `usergroups:write` | [required] |
**usergroup** | **String** | The encoded ID of the User Group to update. | [required] |
**users** | **String** | A comma separated string of encoded user IDs that represent the entire list of users for the User Group. | [required] |
**include_count** | Option<**bool**> | Include the number of users in the User Group. |  |

### Return type

[**crate::models::UsergroupsUsersUpdateSchema**](usergroups_users_update_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

