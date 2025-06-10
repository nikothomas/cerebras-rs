# ChatCompletionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | [**models::ModelIdentifier**](ModelIdentifier.md) |  | 
**messages** | [**Vec<models::ChatMessage>**](ChatMessage.md) | A list of messages comprising the conversation so far | 
**max_tokens** | Option<**u32**> | The maximum number of tokens that can be generated in the completion | [optional]
**temperature** | Option<**f64**> | Sampling temperature to use | [optional][default to 1]
**top_p** | Option<**f64**> | Nucleus sampling parameter | [optional][default to 1]
**stream** | Option<**bool**> | If set, partial message deltas will be sent | [optional][default to false]
**stop** | Option<[**models::StopCondition**](StopCondition.md)> |  | [optional]
**response_format** | Option<[**models::ResponseFormat**](ResponseFormat.md)> |  | [optional]
**tools** | Option<[**Vec<models::Tool>**](Tool.md)> |  | [optional]
**tool_choice** | Option<[**models::ToolChoiceOption**](ToolChoiceOption.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


