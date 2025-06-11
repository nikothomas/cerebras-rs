//! Builder pattern for ChatCompletionRequest

use crate::models::{ChatCompletionRequest, ChatMessage, ModelIdentifier, ResponseFormat, Tool, ToolChoiceOption, StopCondition};
use crate::chat_message::Role;

/// Builder for creating ChatCompletionRequest instances
/// 
/// # Example
/// ```rust,no_run
/// use cerebras_rs::builders::ChatCompletionBuilder;
/// use cerebras_rs::{ChatMessage, ModelIdentifier};
/// 
/// let request = ChatCompletionBuilder::new(ModelIdentifier::Llama3Period18b)
///     .system_message("You are a helpful assistant")
///     .user_message("What is the capital of France?")
///     .temperature(0.7)
///     .max_tokens(100)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ChatCompletionBuilder {
    model: ModelIdentifier,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    stream: Option<bool>,
    stop: Option<Vec<String>>,
    response_format: Option<ResponseFormat>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoiceOption>,
}

impl ChatCompletionBuilder {
    /// Create a new builder with the specified model
    pub fn new(model: ModelIdentifier) -> Self {
        Self {
            model,
            messages: Vec::new(),
            max_tokens: None,
            temperature: None,
            top_p: None,
            stream: None,
            stop: None,
            response_format: None,
            tools: None,
            tool_choice: None,
        }
    }
    
    /// Add a message to the conversation
    pub fn message(mut self, message: ChatMessage) -> Self {
        self.messages.push(message);
        self
    }
    
    /// Add multiple messages to the conversation
    pub fn messages(mut self, messages: impl IntoIterator<Item = ChatMessage>) -> Self {
        self.messages.extend(messages);
        self
    }
    
    /// Add a system message
    pub fn system_message(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::system(content));
        self
    }
    
    /// Add a user message
    pub fn user_message(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::user(content));
        self
    }
    
    /// Add an assistant message
    pub fn assistant_message(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::assistant(content));
        self
    }
    
    /// Set the maximum number of tokens to generate
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    
    /// Set the sampling temperature (0.0 to 2.0)
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }
    
    /// Set the nucleus sampling parameter (0.0 to 1.0)
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }
    
    /// Enable or disable streaming
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
    
    /// Set stop sequences
    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }
    
    /// Add a single stop sequence
    pub fn stop_sequence(mut self, sequence: impl Into<String>) -> Self {
        self.stop.get_or_insert_with(Vec::new).push(sequence.into());
        self
    }
    
    /// Set the response format
    pub fn response_format(mut self, format: ResponseFormat) -> Self {
        self.response_format = Some(format);
        self
    }
    
    /// Set JSON response format with a schema
    pub fn json_response_with_schema(mut self, schema: crate::models::JsonSchema) -> Self {
        self.response_format = Some(ResponseFormat {
            r#type: Some(crate::response_format::Type::JsonSchema),
            json_schema: Some(schema),
        });
        self
    }
    
    /// Set JSON response format with schema details
    pub fn json_schema(mut self, name: impl Into<String>, schema: serde_json::Value, strict: bool) -> Self {
        self.response_format = Some(ResponseFormat {
            r#type: Some(crate::response_format::Type::JsonSchema),
            json_schema: Some(crate::models::JsonSchema {
                name: Some(name.into()),
                schema: Some(schema),
                strict: Some(strict),
            }),
        });
        self
    }
    
    /// Set available tools
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }
    
    /// Add a single tool
    pub fn tool(mut self, tool: Tool) -> Self {
        self.tools.get_or_insert_with(Vec::new).push(tool);
        self
    }
    
    /// Set tool choice
    pub fn tool_choice(mut self, choice: ToolChoiceOption) -> Self {
        self.tool_choice = Some(choice);
        self
    }
    
    /// Build the ChatCompletionRequest
    pub fn build(self) -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: self.model,
            messages: self.messages,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            stream: self.stream,
            stop: self.stop.map(|s| {
                if s.len() == 1 {
                    StopCondition::String(s.into_iter().next().unwrap())
                } else {
                    StopCondition::Array(s)
                }
            }),
            response_format: self.response_format,
            tools: self.tools,
            tool_choice: self.tool_choice,
        }
    }
}

impl ChatCompletionRequest {
    /// Create a new builder for this request type
    pub fn builder(model: ModelIdentifier) -> ChatCompletionBuilder {
        ChatCompletionBuilder::new(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builder_basic() {
        let request = ChatCompletionBuilder::new(ModelIdentifier::Llama3Period18b)
            .user_message("Hello")
            .temperature(0.5)
            .build();
            
        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.messages[0].content, "Hello");
        assert_eq!(request.temperature, Some(0.5));
    }
    
    #[test]
    fn test_builder_multiple_messages() {
        let request = ChatCompletionBuilder::new(ModelIdentifier::Llama3Period18b)
            .system_message("You are helpful")
            .user_message("Hi")
            .assistant_message("Hello!")
            .user_message("How are you?")
            .build();
            
        assert_eq!(request.messages.len(), 4);
        assert_eq!(request.messages[0].role, Role::System);
        assert_eq!(request.messages[1].role, Role::User);
        assert_eq!(request.messages[2].role, Role::Assistant);
        assert_eq!(request.messages[3].role, Role::User);
    }
}
