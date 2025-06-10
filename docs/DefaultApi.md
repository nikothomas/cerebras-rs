# \DefaultApi

All URIs are relative to *https://api.cerebras.ai/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_chat_completion**](DefaultApi.md#create_chat_completion) | **POST** /chat/completions | Create chat completion
[**create_completion**](DefaultApi.md#create_completion) | **POST** /completions | Create text completion
[**list_models**](DefaultApi.md#list_models) | **GET** /models | List available models
[**retrieve_model**](DefaultApi.md#retrieve_model) | **GET** /models/{model} | Retrieve a model



## create_chat_completion

> models::CreateChatCompletion200Response create_chat_completion(chat_completion_request)
Create chat completion

Creates a completion for the chat message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_completion_request** | [**ChatCompletionRequest**](ChatCompletionRequest.md) |  | [required] |

### Return type

[**models::CreateChatCompletion200Response**](createChatCompletion_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_completion

> models::CreateCompletion200Response create_completion(completion_request)
Create text completion

Creates a completion for the provided prompt and parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completion_request** | [**CompletionRequest**](CompletionRequest.md) |  | [required] |

### Return type

[**models::CreateCompletion200Response**](createCompletion_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_models

> models::ModelList list_models()
List available models

Lists the currently available models and provides essential details about each, including the owner and availability.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelList**](ModelList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_model

> models::Model retrieve_model(model)
Retrieve a model

Fetches a model instance, offering key details about the model, including its owner and permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model** | [**ModelIdentifier**](.md) | The model identifier | [required] |

### Return type

[**models::Model**](Model.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

