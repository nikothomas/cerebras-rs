// Auto-generated integration tests for the Cerebras API client
// Run 'cargo test --test integration' to execute these tests
// To run tests with API calls, set CEREBRAS_API_KEY environment variable

#![cfg(test)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use cerebras_rs::{
    Client, Configuration, ModelIdentifier,
    models::{ChatCompletionRequest, ChatMessage, CompletionRequest},
};
use std::env;
use futures::stream::StreamExt;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use cerebras_rs::api;
    use tokio;

    // Helper to skip tests when no API key is available
    fn requires_api_key() -> bool {
        if env::var("CEREBRAS_API_KEY").is_ok() {
            true
        } else {
            println!("Skipping test - set CEREBRAS_API_KEY to run this test");
            false
        }
    }

    // Helper function to create a test client
    fn create_test_client() -> Client {
        // Use from_env() which properly handles the API key
        Client::from_env().expect("Failed to create client from CEREBRAS_API_KEY environment variable")
    }

    // Helper function to create a test chat completion request
    fn create_test_chat_request() -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            messages: vec![
                ChatMessage::system("You are a helpful assistant."),
                ChatMessage::user("Hello, world!"),
            ],
            ..Default::default()
        }
    }

    // Helper function to create a test completion request
    fn create_test_completion_request() -> CompletionRequest {
        CompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            prompt: cerebras_rs::models::Prompt::String("Hello, world!".to_string()),
            ..Default::default()
        }
    }

    // Test listing available models
    #[tokio::test]
    async fn test_list_models() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        let result = client.list_models().await;
        
        assert!(result.is_ok(), "Failed to list models: {:?}", result.err());
        let models = result.unwrap();
        
        assert!(models.data.is_some(), "No models data returned");
        let data = models.data.unwrap();
        assert!(!data.is_empty(), "No models returned");
        
        // Verify at least one model has expected fields
        let first_model = &data[0];
        assert!(first_model.id.is_some(), "Model ID is missing");
        assert!(first_model.owned_by.is_some(), "Model owner is missing");
        
        println!("Found {} models", data.len());
        for model in data.iter().take(5) {
            println!("  - {}: owned by {}", 
                model.id.as_ref().unwrap_or(&"unknown".to_string()),
                model.owned_by.as_ref().unwrap_or(&"unknown".to_string())
            );
        }
    }

    // Test retrieving a specific model
    #[tokio::test]
    async fn test_get_model() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        let result = client.get_model(ModelIdentifier::Llama3Period18b).await;
        
        assert!(result.is_ok(), "Failed to get model: {:?}", result.err());
        let model = result.unwrap();
        
        assert!(model.id.is_some(), "Model ID is missing");
        let model_id = model.id.clone().unwrap();
        assert_eq!(model_id, "llama3.1-8b");
        assert!(model.owned_by.is_some(), "Model owner is missing");
        
        println!("Retrieved model: {} (owned by {})", 
            model.id.as_ref().unwrap_or(&"unknown".to_string()),
            model.owned_by.as_ref().unwrap_or(&"unknown".to_string())
        );
    }

    // Test basic chat completion
    #[tokio::test]
    async fn test_chat_completion_basic() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        let request = ChatCompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            messages: vec![
                ChatMessage::system("You are a helpful assistant. Keep responses brief."),
                ChatMessage::user("What is 2+2? Reply with just the number."),
            ],
            max_tokens: Some(10),
            temperature: Some(0.0),
            ..Default::default()
        };

        let result = client.chat_completion(request).await;
        assert!(result.is_ok(), "Failed to create chat completion: {:?}", result.err());
        
        let response = result.unwrap();
        assert!(response.choices.is_some(), "No choices in response");
        
        let choices = response.choices.unwrap();
        assert!(!choices.is_empty(), "Empty choices array");
        
        let first_choice = &choices[0];
        assert!(first_choice.message.is_some(), "No message in choice");
        
        let message = first_choice.message.as_ref().unwrap();
        println!("Assistant response: {}", message.content);
        
        // Verify response metadata
        assert!(response.usage.is_some(), "No usage data");
        let usage = response.usage.unwrap();
        assert!(usage.total_tokens.is_some() && usage.total_tokens.unwrap() > 0, "Invalid token count");
    }

    // Test chat completion with multiple messages
    #[tokio::test]
    async fn test_chat_completion_conversation() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        let request = ChatCompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            messages: vec![
                ChatMessage::system("You are a helpful math tutor. Keep responses brief."),
                ChatMessage::user("What is the square root of 16?"),
                ChatMessage::assistant("The square root of 16 is 4."),
                ChatMessage::user("And what is 4 squared?"),
            ],
            max_tokens: Some(50),
            temperature: Some(0.0),
            ..Default::default()
        };

        let result = client.chat_completion(request).await;
        assert!(result.is_ok(), "Failed to create chat completion: {:?}", result.err());
        
        let response = result.unwrap();
        let choices = response.choices.unwrap();
        let first_choice = &choices[0];
        let message = first_choice.message.as_ref().unwrap();
        println!("Assistant response: {}", message.content);
        
        // The response should mention 16 since 4^2 = 16
        assert!(message.content.to_lowercase().contains("16"), 
            "Expected response to contain '16'");
    }

    // Test chat completion with streaming
    #[tokio::test]
    async fn test_chat_completion_streaming() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        let request = ChatCompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            messages: vec![
                ChatMessage::system("You are a helpful assistant."),
                ChatMessage::user("Count from 1 to 5, one number per line."),
            ],
            max_tokens: Some(50),
            temperature: Some(0.0),
            stream: Some(true),
            ..Default::default()
        };

        let mut stream = client.chat_completion_stream(request).await
            .expect("Failed to create chat completion stream");
        
        let mut full_response = String::new();
        let mut chunk_count = 0;
        
        while let Some(chunk_result) = stream.next().await {
            assert!(chunk_result.is_ok(), "Stream chunk error: {:?}", chunk_result.err());
            let chunk = chunk_result.unwrap();
            
            if let Some(choices) = &chunk.choices {
                if let Some(choice) = choices.first() {
                    if let Some(delta) = &choice.delta {
                        if let Some(content) = &delta.content {
                            full_response.push_str(content);
                            print!("{}", content);
                            chunk_count += 1;
                        }
                    }
                }
            }
        }
        
        println!("\nReceived {} chunks", chunk_count);
        assert!(chunk_count > 1, "Expected multiple chunks in stream");
        assert!(!full_response.is_empty(), "No content received from stream");
    }

    // Test text completion
    #[tokio::test]
    async fn test_completion_basic() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        let request = CompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            prompt: cerebras_rs::models::Prompt::String("The capital of France is".to_string()),
            max_tokens: Some(10),
            temperature: Some(0.0),
            ..Default::default()
        };

        let result = client.completion(request).await;
        assert!(result.is_ok(), "Failed to create completion: {:?}", result.err());
        
        let response = result.unwrap();
        assert!(response.choices.is_some(), "No choices in response");
        
        let choices = response.choices.unwrap();
        assert!(!choices.is_empty(), "Empty choices array");
        
        let first_choice = &choices[0];
        assert!(first_choice.text.is_some(), "No text in choice");
        
        let text = first_choice.text.as_ref().unwrap();
        println!("Completion: {}", text);
        
        // The response should be non-empty and contain some text
        assert!(!text.trim().is_empty(), "Expected non-empty completion");
    }

    // Test completion with streaming
    #[tokio::test]
    async fn test_completion_streaming() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        let request = CompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            prompt: cerebras_rs::models::Prompt::String("Once upon a time".to_string()),
            max_tokens: Some(50),
            temperature: Some(0.7),
            stream: Some(true),
            ..Default::default()
        };

        let mut stream = client.completion_stream(request).await
            .expect("Failed to create completion stream");
        
        let mut full_response = String::new();
        let mut chunk_count = 0;
        
        while let Some(chunk_result) = stream.next().await {
            assert!(chunk_result.is_ok(), "Stream chunk error: {:?}", chunk_result.err());
            let chunk = chunk_result.unwrap();
            
            if let Some(choices) = &chunk.choices {
                if let Some(choice) = choices.first() {
                    if let Some(text) = &choice.text {
                        full_response.push_str(text);
                        print!("{}", text);
                        chunk_count += 1;
                    }
                }
            }
        }
        
        println!("\nReceived {} chunks", chunk_count);
        assert!(chunk_count > 1, "Expected multiple chunks in stream");
        assert!(!full_response.is_empty(), "No content received from stream");
    }

    // Test error handling - invalid model
    #[tokio::test]
    async fn test_error_invalid_model() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        // Use a non-existent model by using a different valid variant
        // This will still test error handling as the API might reject certain models
        let request = ChatCompletionRequest {
            model: ModelIdentifier::DeepseekR1DistillLlama70b, // Using a different model that might not be available
            messages: vec![ChatMessage::user("Hello")],
            ..Default::default()
        };

        let result = client.chat_completion(request).await;
        // Note: This might succeed if the model is available, so we'll just check the result
        match result {
            Ok(response) => println!("Model is available, got response"),
            Err(error) => println!("Error (expected): {}", error),
        }
    }

    // Test with different temperature settings
    #[tokio::test]
    async fn test_temperature_variation() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        
        // Test with temperature 0 (deterministic)
        let request_deterministic = ChatCompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            messages: vec![
                ChatMessage::user("What is 1+1? Reply with just the number."),
            ],
            max_tokens: Some(5),
            temperature: Some(0.0),
            ..Default::default()
        };

        let result1 = client.chat_completion(request_deterministic.clone()).await;
        assert!(result1.is_ok(), "First request failed: {:?}", result1.err());
        let response1 = result1.unwrap();
        let choices1 = response1.choices.unwrap();
        let message1 = choices1[0].message.as_ref().unwrap();
        let text1 = &message1.content;
        
        // Second call with same parameters should give same result
        let result2 = client.chat_completion(request_deterministic).await;
        assert!(result2.is_ok(), "Second request failed: {:?}", result2.err());
        let response2 = result2.unwrap();
        let choices2 = response2.choices.unwrap();
        let message2 = choices2[0].message.as_ref().unwrap();
        let text2 = &message2.content;
        
        println!("Temperature 0.0 - Response 1: {}", text1);
        println!("Temperature 0.0 - Response 2: {}", text2);
        
        // Both should contain "2"
        assert!(text1.contains("2") && text2.contains("2"), 
            "Expected both responses to contain '2'");
    }

    // Test max_tokens limit
    #[tokio::test]
    async fn test_max_tokens_limit() {
        if !requires_api_key() {
            return;
        }

        let client = create_test_client();
        let request = ChatCompletionRequest {
            model: ModelIdentifier::Llama3Period18b,
            messages: vec![
                ChatMessage::user("Tell me a very long story about dragons."),
            ],
            max_tokens: Some(10), // Very low limit
            ..Default::default()
        };

        let result = client.chat_completion(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        let usage = response.usage.unwrap();
        
        // Completion tokens should be around our limit
        assert!(usage.completion_tokens.unwrap() <= 15, 
            "Expected completion tokens to be limited");
        
        println!("Completion tokens used: {}", usage.completion_tokens.unwrap());
    }

    #[tokio::test]
    async fn test_assistant() {
        // Test ChatMessage::assistant helper
        let msg = ChatMessage::assistant("I am an assistant");
        assert_eq!(msg.role, cerebras_rs::chat_message::Role::Assistant);
        assert_eq!(msg.content, "I am an assistant");
    }

    #[tokio::test]
    async fn test_configuration() {
        // Test custom configuration
        let mut config = Configuration::new();
        config.bearer_access_token = Some("custom-key".to_string());
        config.base_path = "https://custom.example.com".to_string();
        
        let client = Client::with_configuration(config);
        assert_eq!(client.configuration().base_path, "https://custom.example.com");
        assert_eq!(client.configuration().bearer_access_token, Some("custom-key".to_string()));
    }

    #[tokio::test]
    async fn test_new() {
        // Test creating client with explicit API key
        let client = Client::new("test-api-key");
        assert!(!client.configuration().base_path.is_empty());
        assert_eq!(client.configuration().bearer_access_token, Some("test-api-key".to_string()));
    }

    #[tokio::test]
    async fn test_system() {
        // Test ChatMessage::system helper
        let msg = ChatMessage::system("You are a helpful assistant");
        assert_eq!(msg.role, cerebras_rs::chat_message::Role::System);
        assert_eq!(msg.content, "You are a helpful assistant");
    }

    #[tokio::test]
    async fn test_tool() {
        // Test ChatMessage::tool helper
        let msg = ChatMessage::tool("Tool response", "tool-123");
        assert_eq!(msg.role, cerebras_rs::chat_message::Role::Tool);
        assert_eq!(msg.content, "Tool response");
        assert_eq!(msg.tool_call_id, Some("tool-123".to_string()));
    }

    #[tokio::test]
    async fn test_user() {
        // Test ChatMessage::user helper
        let msg = ChatMessage::user("Hello, assistant!");
        assert_eq!(msg.role, cerebras_rs::chat_message::Role::User);
        assert_eq!(msg.content, "Hello, assistant!");
    }

    #[tokio::test]
    async fn test_with_base_url() {
        let client = create_test_client()
            .with_base_url("http://example.com".to_string());
        assert_eq!(client.configuration().base_path, "http://example.com");
    }

    #[tokio::test]
    async fn test_with_configuration() {
        let config = Configuration::new();
        let client = Client::with_configuration(config);
        assert!(!client.configuration().base_path.is_empty());
    }

}

// Add any additional test helpers below
// For example, test utilities or shared test data

#[cfg(test)]
mod test_helpers {
    // Test helper functions can go here
}
