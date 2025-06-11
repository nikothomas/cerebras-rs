//! Streaming support for Cerebras API responses

use futures_util::{Stream, StreamExt};
use eventsource_stream::Eventsource;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::{
    apis::{configuration::Configuration, default_api},
    models::*,
    Error, Result,
};

pin_project! {
    /// Stream handler for chat completion responses
    pub struct ChatCompletionStream {
        #[pin]
        inner: Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk>> + Send>>,
    }
}

impl ChatCompletionStream {
    /// Create a new chat completion stream
    pub async fn new(
        configuration: &Configuration,
        request: ChatCompletionRequest,
    ) -> Result<Self> {
        // Ensure streaming is enabled
        let mut request = request;
        request.stream = Some(true);
        
        // Make the request
        let response = configuration.client
            .post(&format!("{}/chat/completions", configuration.base_path))
            .bearer_auth(configuration.bearer_access_token.as_ref().ok_or_else(|| {
                Error::Configuration("No API key configured".into())
            })?)
            .json(&request)
            .send()
            .await
            .map_err(Error::Http)?;
            
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::Api(format!("HTTP {}: {}", status, text)));
        }
        
        // Create event stream
        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|result| async move {
                match result {
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str::<ChatCompletionChunk>(&event.data) {
                                Ok(chunk) => Some(Ok(chunk)),
                                Err(e) => Some(Err(Error::Serialization(e))),
                            }
                        }
                    }
                    Err(e) => Some(Err(Error::Streaming(format!("Event stream error: {}", e)))),
                }
            });
            
        Ok(Self {
            inner: Box::pin(stream),
        })
    }
    
    /// Collect all chunks into a complete response
    pub async fn collect(mut self) -> Result<ChatCompletion> {
        let mut messages = Vec::new();
        let mut model = None;
        let mut id = None;
        let mut created = None;
        let mut finish_reason = None;
        
        while let Some(chunk) = self.next().await {
            let chunk = chunk?;
            
            if id.is_none() && chunk.id.is_some() {
                id = chunk.id;
            }
            if model.is_none() && chunk.model.is_some() {
                model = chunk.model;
            }
            if created.is_none() && chunk.created.is_some() {
                created = chunk.created;
            }
            
            if let Some(choices) = chunk.choices {
                for choice in choices {
                    if let Some(delta) = choice.delta {
                        if let Some(content) = delta.content {
                            messages.push(content);
                        }
                    }
                    if choice.finish_reason.is_some() {
                        finish_reason = choice.finish_reason.map(|fr| match fr {
                            crate::models::chat_choice_delta::FinishReason::Stop => {
                                crate::models::chat_choice::FinishReason::Stop
                            }
                            crate::models::chat_choice_delta::FinishReason::Length => {
                                crate::models::chat_choice::FinishReason::Length
                            }
                            crate::models::chat_choice_delta::FinishReason::ToolCalls => {
                                crate::models::chat_choice::FinishReason::ToolCalls
                            }
                            crate::models::chat_choice_delta::FinishReason::ContentFilter => {
                                crate::models::chat_choice::FinishReason::ContentFilter
                            }
                        });
                    }
                }
            }
        }
        
        Ok(ChatCompletion {
            id,
            object: Some(crate::models::chat_completion::Object::ChatPeriodCompletion),
            created,
            model,
            system_fingerprint: None,
            choices: Some(vec![ChatChoice {
                index: Some(0),
                message: Some(ChatMessage {
                    role: crate::models::chat_message::Role::Assistant,
                    content: messages.join(""),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                }),
                finish_reason,
            }]),
            usage: None,
            time_info: None,
        })
    }
}

impl Stream for ChatCompletionStream {
    type Item = Result<ChatCompletionChunk>;
    
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(cx)
    }
}

pin_project! {
    /// Stream handler for completion responses
    pub struct CompletionStream {
        #[pin]
        inner: Pin<Box<dyn Stream<Item = Result<CompletionChunk>> + Send>>,
    }
}

impl CompletionStream {
    /// Create a new completion stream
    pub async fn new(
        configuration: &Configuration,
        request: CompletionRequest,
    ) -> Result<Self> {
        // Ensure streaming is enabled
        let mut request = request;
        request.stream = Some(true);
        
        // Make the request
        let response = configuration.client
            .post(&format!("{}/completions", configuration.base_path))
            .bearer_auth(configuration.bearer_access_token.as_ref().ok_or_else(|| {
                Error::Configuration("No API key configured".into())
            })?)
            .json(&request)
            .send()
            .await
            .map_err(Error::Http)?;
            
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::Api(format!("HTTP {}: {}", status, text)));
        }
        
        // Create event stream
        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|result| async move {
                match result {
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str::<CompletionChunk>(&event.data) {
                                Ok(chunk) => Some(Ok(chunk)),
                                Err(e) => Some(Err(Error::Serialization(e))),
                            }
                        }
                    }
                    Err(e) => Some(Err(Error::Streaming(format!("Event stream error: {}", e)))),
                }
            });
            
        Ok(Self {
            inner: Box::pin(stream),
        })
    }
    
    /// Collect all chunks into a complete response
    pub async fn collect(mut self) -> Result<Completion> {
        let mut texts = Vec::new();
        let mut model = None;
        let mut id = None;
        let mut created = None;
        let mut finish_reason = None;
        
        while let Some(chunk) = self.next().await {
            let chunk = chunk?;
            
            if id.is_none() && chunk.id.is_some() {
                id = chunk.id;
            }
            if model.is_none() && chunk.model.is_some() {
                model = chunk.model;
            }
            if created.is_none() && chunk.created.is_some() {
                created = chunk.created;
            }
            
            if let Some(choices) = chunk.choices {
                for choice in choices {
                    if let Some(text) = choice.text {
                        texts.push(text);
                    }
                    if choice.finish_reason.is_some() {
                        finish_reason = choice.finish_reason.map(|fr| match fr {
                            crate::models::completion_choice_delta::FinishReason::Stop => {
                                crate::models::completion_choice::FinishReason::Stop
                            }
                            crate::models::completion_choice_delta::FinishReason::Length => {
                                crate::models::completion_choice::FinishReason::Length
                            }
                        });
                    }
                }
            }
        }
        
        Ok(Completion {
            id,
            object: Some(crate::models::completion::Object::TextCompletion),
            created,
            model,
            system_fingerprint: None,
            choices: Some(vec![CompletionChoice {
                index: Some(0),
                text: Some(texts.join("")),
                finish_reason,
            }]),
            usage: None,
            time_info: None,
        })
    }
}

impl Stream for CompletionStream {
    type Item = Result<CompletionChunk>;
    
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Note: Actual streaming tests would require a mock server
    // These are just compilation tests
    
    #[test]
    fn test_stream_types() {
        // Ensure the types compile correctly
        fn assert_send<T: Send>() {}
        fn assert_stream<T: Stream>() {}
        
        assert_send::<ChatCompletionStream>();
        assert_stream::<ChatCompletionStream>();
        assert_send::<CompletionStream>();
        assert_stream::<CompletionStream>();
    }
}
