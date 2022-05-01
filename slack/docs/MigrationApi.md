# \MigrationApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**migration_exchange**](MigrationApi.md#migration_exchange) | **GET** /migration.exchange | 



## migration_exchange

> crate::models::MigrationExchangeSuccessSchema migration_exchange(token, users, team_id, to_old)


For Enterprise Grid workspaces, map local user IDs to global user IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `tokens.basic` | [required] |
**users** | **String** | A comma-separated list of user ids, up to 400 per request | [required] |
**team_id** | Option<**String**> | Specify team_id starts with `T` in case of Org Token |  |
**to_old** | Option<**bool**> | Specify `true` to convert `W` global user IDs to workspace-specific `U` IDs. Defaults to `false`. |  |

### Return type

[**crate::models::MigrationExchangeSuccessSchema**](migration_exchange_success_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

