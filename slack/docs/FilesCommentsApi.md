# \FilesCommentsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_comments_delete**](FilesCommentsApi.md#files_comments_delete) | **POST** /files.comments.delete | 



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

