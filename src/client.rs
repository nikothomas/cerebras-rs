//! High-level client for the Cerebras Inference API
//! 
//! This module provides an ergonomic client interface that wraps the generated API code
//! with additional conveniences like builder patterns and streaming support.

use crate::{
    apis::{configuration::Configuration, default_api, ResponseContent},
    models::*,
    chat_message::Role,
    Error, Result,
};

/// High-level client for interacting with the Cerebras Inference API
/// 
/// # Example
/// ```rust,no_run
/// use cerebras_rs::Client;
/// 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::from_env()?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Client {
    configuration: Configuration,
}

impl Client {
    /// Create a new client with the provided API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(api_key.into());
        
        Self { configuration }
    }
    
    /// Create a new client from the CEREBRAS_API_KEY environment variable
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("CEREBRAS_API_KEY")
            .map_err(|_| Error::Configuration("CEREBRAS_API_KEY environment variable not set".into()))?;
        Ok(Self::new(api_key))
    }
    
    /// Create a new client with a custom configuration
    pub fn with_configuration(configuration: Configuration) -> Self {
        Self { configuration }
    }
    
    /// Set a custom base URL (useful for testing or proxies)
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.configuration.base_path = base_url;
        self
    }
    
    /// Get a reference to the underlying configuration
    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
    
    /// List available models
    /// 
    /// # Example
    /// ```rust,no_run
    /// # use cerebras_rs::Client;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::from_env()?;
    /// let models = client.list_models().await?;
    /// if let Some(data) = &models.data {
    ///     for model in data {
    ///         println!("{}: {}", 
    ///             model.id.as_ref().unwrap_or(&"unknown".to_string()), 
    ///             model.owned_by.as_ref().unwrap_or(&"unknown".to_string()));
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_models(&self) -> Result<ModelList> {
        let response = default_api::list_models(&self.configuration).await?;
        match response.entity {
            Some(default_api::ListModelsSuccess::Status200(models)) => Ok(models),
            _ => Err(Error::Api("Unexpected response format".into())),
        }
    }
    
    /// Retrieve details about a specific model
    pub async fn get_model(&self, model: ModelIdentifier) -> Result<Model> {
        let response = default_api::retrieve_model(&self.configuration, model).await?;
        match response.entity {
            Some(default_api::RetrieveModelSuccess::Status200(model)) => Ok(model),
            _ => Err(Error::Api("Unexpected response format".into())),
        }
    }
    
    /// Create a chat completion
    /// 
    /// # Example
    /// ```rust,no_run
    /// # use cerebras_rs::{Client, ChatCompletionRequest, ChatMessage, ModelIdentifier};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::from_env()?;
    /// let request = ChatCompletionRequest {
    ///     model: ModelIdentifier::Llama3Period18b,
    ///     messages: vec![
    ///         ChatMessage::system("You are a helpful assistant"),
    ///         ChatMessage::user("What is the capital of France?"),
    ///     ],
    ///     ..Default::default()
    /// };
    /// 
    /// let response = client.chat_completion(request).await?;
    /// if let Some(choices) = &response.choices {
    ///     if let Some(first_choice) = choices.first() {
    ///         if let Some(message) = &first_choice.message {
    ///             println!("{}", message.content);
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn chat_completion(&self, request: ChatCompletionRequest) -> Result<CreateChatCompletionResponse> {
        let response = default_api::create_chat_completion(&self.configuration, request).await?;
        match response.entity {
            Some(default_api::CreateChatCompletionSuccess::Status200(resp)) => {
                match resp {
                    CreateChatCompletion200Response::CreateChatCompletionResponse(completion) => Ok(completion),
                    CreateChatCompletion200Response::ChatCompletionChunk(_) => {
                        Err(Error::Api("Unexpected streaming response for non-streaming request".into()))
                    }
                }
            }
            _ => Err(Error::Api("Unexpected response format".into())),
        }
    }
    
    /// Create a chat completion with streaming
    /// 
    /// # Example
    /// ```rust,no_run
    /// # use cerebras_rs::{Client, ChatCompletionRequest, ChatMessage, ModelIdentifier};
    /// # use futures_util::StreamExt;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::from_env()?;
    /// let request = ChatCompletionRequest {
    ///     model: ModelIdentifier::Llama3Period18b,
    ///     messages: vec![ChatMessage::user("Tell me a story")],
    ///     stream: Some(true),
    ///     ..Default::default()
    /// };
    /// 
    /// let mut stream = client.chat_completion_stream(request).await?;
    /// while let Some(chunk) = stream.next().await {
    ///     match chunk {
    ///         Ok(chunk) => print!("{}", chunk.choices[0].delta.content.as_deref().unwrap_or("")),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "stream")]
    pub async fn chat_completion_stream(
        &self,
        mut request: ChatCompletionRequest,
    ) -> Result<crate::streaming::ChatCompletionStream> {
        request.stream = Some(true);
        crate::streaming::ChatCompletionStream::new(&self.configuration, request).await
    }
    
    /// Create a text completion
    pub async fn completion(&self, request: CompletionRequest) -> Result<CreateCompletionResponse> {
        let response = default_api::create_completion(&self.configuration, request).await?;
        match response.entity {
            Some(default_api::CreateCompletionSuccess::Status200(resp)) => {
                match resp {
                    CreateCompletion200Response::CreateCompletionResponse(completion) => Ok(completion),
                    CreateCompletion200Response::CompletionChunk(_) => {
                        Err(Error::Api("Unexpected streaming response for non-streaming request".into()))
                    }
                }
            }
            _ => Err(Error::Api("Unexpected response format".into())),
        }
    }
    
    /// Create a text completion with streaming
    #[cfg(feature = "stream")]
    pub async fn completion_stream(
        &self,
        mut request: CompletionRequest,
    ) -> Result<crate::streaming::CompletionStream> {
        request.stream = Some(true);
        crate::streaming::CompletionStream::new(&self.configuration, request).await
    }
}

// Convenience methods for ChatMessage
impl ChatMessage {
    /// Create a system message
    pub fn system<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
    
    /// Create a user message
    pub fn user<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
    
    /// Create an assistant message
    pub fn assistant<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
    
    /// Create a tool message
    pub fn tool<S: Into<String>>(content: S, tool_call_id: S) -> Self {
        Self {
            role: Role::Tool,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_client_creation() {
        let client = Client::new("test-key");
        assert_eq!(client.configuration.bearer_access_token, Some("test-key".to_string()));
    }
    
    #[test]
    fn test_chat_message_helpers() {
        let system = ChatMessage::system("You are helpful");
        assert_eq!(system.role, Role::System);
        assert_eq!(system.content, "You are helpful");
        
        let user = ChatMessage::user("Hello");
        assert_eq!(user.role, Role::User);
        assert_eq!(user.content, "Hello");
    }
}
