# \CallsParticipantsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calls_participants_add**](CallsParticipantsApi.md#calls_participants_add) | **POST** /calls.participants.add | 
[**calls_participants_remove**](CallsParticipantsApi.md#calls_participants_remove) | **POST** /calls.participants.remove | 



## calls_participants_add

> crate::models::DefaultSuccessTemplate calls_participants_add(token, id, users)


Registers new participants added to a Call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `calls:write` | [required] |
**id** | **String** | `id` returned by the [`calls.add`](/methods/calls.add) method. | [required] |
**users** | **String** | The list of users to add as participants in the Call. [Read more on how to specify users here](/apis/calls#users). | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calls_participants_remove

> crate::models::DefaultSuccessTemplate calls_participants_remove(token, id, users)


Registers participants removed from a Call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `calls:write` | [required] |
**id** | **String** | `id` returned by the [`calls.add`](/methods/calls.add) method. | [required] |
**users** | **String** | The list of users to remove as participants in the Call. [Read more on how to specify users here](/apis/calls#users). | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

