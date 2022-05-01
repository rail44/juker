# \FilesApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_comments_delete**](FilesApi.md#files_comments_delete) | **POST** /files.comments.delete | 
[**files_delete**](FilesApi.md#files_delete) | **POST** /files.delete | 
[**files_info**](FilesApi.md#files_info) | **GET** /files.info | 
[**files_list**](FilesApi.md#files_list) | **GET** /files.list | 
[**files_remote_add**](FilesApi.md#files_remote_add) | **POST** /files.remote.add | 
[**files_remote_info**](FilesApi.md#files_remote_info) | **GET** /files.remote.info | 
[**files_remote_list**](FilesApi.md#files_remote_list) | **GET** /files.remote.list | 
[**files_remote_remove**](FilesApi.md#files_remote_remove) | **POST** /files.remote.remove | 
[**files_remote_share**](FilesApi.md#files_remote_share) | **GET** /files.remote.share | 
[**files_remote_update**](FilesApi.md#files_remote_update) | **POST** /files.remote.update | 
[**files_revoke_public_url**](FilesApi.md#files_revoke_public_url) | **POST** /files.revokePublicURL | 
[**files_shared_public_url**](FilesApi.md#files_shared_public_url) | **POST** /files.sharedPublicURL | 
[**files_upload**](FilesApi.md#files_upload) | **POST** /files.upload | 



## files_comments_delete

> crate::models::FilesCommentsDeleteSchema files_comments_delete(token, file, id)


Deletes an existing comment on a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | File to delete a comment from. |  |
**id** | Option<**String**> | The comment to delete. |  |

### Return type

[**crate::models::FilesCommentsDeleteSchema**](files_comments_delete_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_delete

> crate::models::FilesDeleteSchema files_delete(token, file)


Deletes a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | ID of file to delete. |  |

### Return type

[**crate::models::FilesDeleteSchema**](files_delete_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_info

> crate::models::FilesInfoSchema files_info(token, file, count, page, limit, cursor)


Gets information about a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:read` |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |
**count** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |
**cursor** | Option<**String**> | Parameter for pagination. File comments are paginated for a single file. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection of comments. See [pagination](/docs/pagination) for more details. |  |

### Return type

[**crate::models::FilesInfoSchema**](files_info_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_list

> crate::models::FilesListSchema files_list(token, user, channel, ts_from, ts_to, types, count, page, show_files_hidden_by_limit)


List for a team, in a channel, or from a user with applied filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:read` |  |
**user** | Option<**String**> | Filter files created by a single user. |  |
**channel** | Option<**String**> | Filter files appearing in a specific channel, indicated by its ID. |  |
**ts_from** | Option<**f32**> | Filter files created after this timestamp (inclusive). |  |
**ts_to** | Option<**f32**> | Filter files created before this timestamp (inclusive). |  |
**types** | Option<**String**> | Filter files by type ([see below](#file_types)). You can pass multiple values in the types argument, like `types=spaces,snippets`.The default value is `all`, which does not filter the list. |  |
**count** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**show_files_hidden_by_limit** | Option<**bool**> | Show truncated file info for files hidden due to being too old, and the team who owns the file being over the file limit. |  |

### Return type

[**crate::models::FilesListSchema**](files_list_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## files_revoke_public_url

> crate::models::FilesRevokePublicUrlSchema files_revoke_public_url(token, file)


Revokes public/external sharing access for a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | File to revoke |  |

### Return type

[**crate::models::FilesRevokePublicUrlSchema**](files_revokePublicURL_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_shared_public_url

> crate::models::FilesSharedPublicUrlSchema files_shared_public_url(token, file)


Enables a file for public/external sharing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | File to share |  |

### Return type

[**crate::models::FilesSharedPublicUrlSchema**](files_sharedPublicURL_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_upload

> crate::models::FilesUploadSchema files_upload(token, file, content, filetype, filename, title, initial_comment, channels, thread_ts)


Uploads or creates a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | File contents via `multipart/form-data`. If omitting this parameter, you must submit `content`. |  |
**content** | Option<**String**> | File contents via a POST variable. If omitting this parameter, you must provide a `file`. |  |
**filetype** | Option<**String**> | A [file type](/types/file#file_types) identifier. |  |
**filename** | Option<**String**> | Filename of file. |  |
**title** | Option<**String**> | Title of file. |  |
**initial_comment** | Option<**String**> | The message text introducing the file in specified `channels`. |  |
**channels** | Option<**String**> | Comma-separated list of channel names or IDs where the file will be shared. |  |
**thread_ts** | Option<**f32**> | Provide another message's `ts` value to upload this file as a reply. Never use a reply's `ts` value; use its parent instead. |  |

### Return type

[**crate::models::FilesUploadSchema**](files_upload_schema.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

