# \WorkflowsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workflows_step_completed**](WorkflowsApi.md#workflows_step_completed) | **GET** /workflows.stepCompleted | 
[**workflows_step_failed**](WorkflowsApi.md#workflows_step_failed) | **GET** /workflows.stepFailed | 
[**workflows_update_step**](WorkflowsApi.md#workflows_update_step) | **GET** /workflows.updateStep | 



## workflows_step_completed

> crate::models::DefaultSuccessTemplate workflows_step_completed(token, workflow_step_execute_id, outputs)


Indicate that an app's step in a workflow completed execution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `workflow.steps:execute` | [required] |
**workflow_step_execute_id** | **String** | Context identifier that maps to the correct workflow step execution. | [required] |
**outputs** | Option<**String**> | Key-value object of outputs from your step. Keys of this object reflect the configured `key` properties of your [`outputs`](/reference/workflows/workflow_step#output) array from your `workflow_step` object. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workflows_step_failed

> crate::models::DefaultSuccessTemplate workflows_step_failed(token, workflow_step_execute_id, error)


Indicate that an app's step in a workflow failed to execute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `workflow.steps:execute` | [required] |
**workflow_step_execute_id** | **String** | Context identifier that maps to the correct workflow step execution. | [required] |
**error** | **String** | A JSON-based object with a `message` property that should contain a human readable error message. | [required] |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workflows_update_step

> crate::models::DefaultSuccessTemplate workflows_update_step(token, workflow_step_edit_id, inputs, outputs, step_name, step_image_url)


Update the configuration for a workflow extension step.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `workflow.steps:execute` | [required] |
**workflow_step_edit_id** | **String** | A context identifier provided with `view_submission` payloads used to call back to `workflows.updateStep`. | [required] |
**inputs** | Option<**String**> | A JSON key-value map of inputs required from a user during configuration. This is the data your app expects to receive when the workflow step starts. **Please note**: the embedded variable format is set and replaced by the workflow system. You cannot create custom variables that will be replaced at runtime. [Read more about variables in workflow steps here](/workflows/steps#variables). |  |
**outputs** | Option<**String**> | An JSON array of output objects used during step execution. This is the data your app agrees to provide when your workflow step was executed. |  |
**step_name** | Option<**String**> | An optional field that can be used to override the step name that is shown in the Workflow Builder. |  |
**step_image_url** | Option<**String**> | An optional field that can be used to override app image that is shown in the Workflow Builder. |  |

### Return type

[**crate::models::DefaultSuccessTemplate**](Default_success_template.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

