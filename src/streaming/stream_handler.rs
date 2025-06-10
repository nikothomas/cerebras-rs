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
        inner: Box<dyn Stream<Item = Result<ChatCompletionChunk>> + Send + Unpin>,
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
            inner: Box::new(stream),
        })
    }
    
    /// Collect all chunks into a complete response
    pub async fn collect(mut self) -> Result<ChatCompletion> {
        let mut messages = Vec::new();
        let mut model = String::new();
        let mut id = String::new();
        let mut created = 0;
        let mut finish_reason = None;
        
        while let Some(chunk) = self.next().await {
            let chunk = chunk?;
            
            if id.is_empty() {
                id = chunk.id;
            }
            if model.is_empty() {
                model = chunk.model;
            }
            if created == 0 {
                created = chunk.created;
            }
            
            for choice in chunk.choices {
                if let Some(content) = choice.delta.content {
                    messages.push(content);
                }
                if choice.finish_reason.is_some() {
                    finish_reason = choice.finish_reason;
                }
            }
        }
        
        Ok(ChatCompletion {
            id,
            object: "chat.completion".to_string(),
            created,
            model,
            system_fingerprint: None,
            choices: vec![ChatChoice {
                index: 0,
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: messages.join(""),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
                finish_reason,
            }],
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
        inner: Box<dyn Stream<Item = Result<CompletionChunk>> + Send + Unpin>,
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
            inner: Box::new(stream),
        })
    }
    
    /// Collect all chunks into a complete response
    pub async fn collect(mut self) -> Result<Completion> {
        let mut texts = Vec::new();
        let mut model = String::new();
        let mut id = String::new();
        let mut created = 0;
        let mut finish_reason = None;
        
        while let Some(chunk) = self.next().await {
            let chunk = chunk?;
            
            if id.is_empty() {
                id = chunk.id;
            }
            if model.is_empty() {
                model = chunk.model;
            }
            if created == 0 {
                created = chunk.created;
            }
            
            for choice in chunk.choices {
                if let Some(text) = choice.text {
                    texts.push(text);
                }
                if choice.finish_reason.is_some() {
                    finish_reason = choice.finish_reason;
                }
            }
        }
        
        Ok(Completion {
            id,
            object: "text_completion".to_string(),
            created,
            model,
            system_fingerprint: None,
            choices: vec![CompletionChoice {
                index: 0,
                text: texts.join(""),
                finish_reason,
            }],
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
