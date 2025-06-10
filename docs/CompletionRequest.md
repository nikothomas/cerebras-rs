# CompletionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | [**models::ModelIdentifier**](ModelIdentifier.md) |  | 
**prompt** | [**models::Prompt**](Prompt.md) |  | 
**max_tokens** | Option<**u32**> | The maximum number of tokens to generate | [optional]
**temperature** | Option<**f64**> |  | [optional][default to 1]
**top_p** | Option<**f64**> |  | [optional][default to 1]
**stream** | Option<**bool**> |  | [optional][default to false]
**stop** | Option<[**models::StopCondition**](StopCondition.md)> |  | [optional]
**return_raw_tokens** | Option<**bool**> | Return raw tokens instead of text | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


