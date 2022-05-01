# \FilesRemoteApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_remote_add**](FilesRemoteApi.md#files_remote_add) | **POST** /files.remote.add | 
[**files_remote_info**](FilesRemoteApi.md#files_remote_info) | **GET** /files.remote.info | 
[**files_remote_list**](FilesRemoteApi.md#files_remote_list) | **GET** /files.remote.list | 
[**files_remote_remove**](FilesRemoteApi.md#files_remote_remove) | **POST** /files.remote.remove | 
[**files_remote_share**](FilesRemoteApi.md#files_remote_share) | **GET** /files.remote.share | 
[**files_remote_update**](FilesRemoteApi.md#files_remote_update) | **POST** /files.remote.update | 



## files_remote_add

> crate::models::DefaultSuccessTemplate files_remote_add(token, external_id, title, filetype, external_url, preview_image, indexable_file_contents)


Adds a file from a remote service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:write` |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |
**title** | Option<**String**> | Title of the file being shared. |  |
**filetype** | Option<**String**> | type of file |  |
**external_url** | Option<**String**> | URL of the remote file. |  |
**preview_image** | Option<**String**> | Preview of the document via `multipart/form-data`. |  |
**indexable_file_contents** | Option<**String**> | A text file (txt, pdf, doc, etc.) containing textual search terms that are used to improve discovery of the remote file. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_info

> crate::models::DefaultSuccessTemplate files_remote_info(token, file, external_id)


Retrieve information about a remote file added to Slack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:read` |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_list

> crate::models::DefaultSuccessTemplate files_remote_list(token, channel, ts_from, ts_to, limit, cursor)


Retrieve information about a remote file added to Slack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:read` |  |
**channel** | Option<**String**> | Filter files appearing in a specific channel, indicated by its ID. |  |
**ts_from** | Option<**f32**> | Filter files created after this timestamp (inclusive). |  |
**ts_to** | Option<**f32**> | Filter files created before this timestamp (inclusive). |  |
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_remove

> crate::models::DefaultSuccessTemplate files_remote_remove(token, file, external_id)


Remove a remote file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:write` |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_share

> crate::models::DefaultSuccessTemplate files_remote_share(token, file, external_id, channels)


Share a remote file into a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:share` |  |
**file** | Option<**String**> | Specify a file registered with Slack by providing its ID. Either this field or `external_id` or both are required. |  |
**external_id** | Option<**String**> | The globally unique identifier (GUID) for the file, as set by the app registering the file with Slack.  Either this field or `file` or both are required. |  |
**channels** | Option<**String**> | Comma-separated list of channel IDs where the file will be shared. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_update

> crate::models::DefaultSuccessTemplate files_remote_update(token, file, external_id, title, filetype, external_url, preview_image, indexable_file_contents)


Updates an existing remote file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:write` |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |
**title** | Option<**String**> | Title of the file being shared. |  |
**filetype** | Option<**String**> | type of file |  |
**external_url** | Option<**String**> | URL of the remote file. |  |
**preview_image** | Option<**String**> | Preview of the document via `multipart/form-data`. |  |
**indexable_file_contents** | Option<**String**> | File containing contents that can be used to improve searchability for the remote file. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

