# \RtmApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rtm_connect**](RtmApi.md#rtm_connect) | **GET** /rtm.connect | 



## rtm_connect

> crate::models::RtmConnectSchema rtm_connect(token, batch_presence_aware, presence_sub)


Starts a Real Time Messaging session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `rtm:stream` | [required] |
**batch_presence_aware** | Option<**bool**> | Batch presence deliveries via subscription. Enabling changes the shape of `presence_change` events. See [batch presence](/docs/presence-and-status#batching). |  |
**presence_sub** | Option<**bool**> | Only deliver presence events when requested by subscription. See [presence subscriptions](/docs/presence-and-status#subscriptions). |  |

### Return type

[**crate::models::RtmConnectSchema**](rtm_connect_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

