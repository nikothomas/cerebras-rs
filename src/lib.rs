#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

//! # Cerebras Rust SDK
//! 
//! High-performance Rust SDK for the Cerebras Inference API, providing low-latency
//! AI model inference powered by Cerebras Wafer-Scale Engines and CS-3 systems.
//! 
//! ## Features
//! 
//! - **Async/await support** - Built on tokio for high-performance async operations
//! - **Streaming responses** - Real-time token streaming for chat and completions
//! - **Type-safe API** - Strongly typed requests and responses
//! - **Builder patterns** - Ergonomic API for constructing requests
//! - **Comprehensive error handling** - Detailed error types for all failure modes
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use cerebras_rs::{Client, ChatCompletionRequest, ChatMessage, ModelIdentifier};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(std::env::var("CEREBRAS_API_KEY")?);
//!     
//!     let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
//!         .user_message("Hello, how are you?")
//!         .build();
//!     
//!     let response = client.chat_completion(request).await?;
//!     if let Some(choices) = &response.choices {
//!         if let Some(choice) = choices.first() {
//!             if let Some(message) = &choice.message {
//!                 println!("{}", message.content);
//!             }
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! You can also directly use the API client:
//! 
//! ```rust,no_run
//! use cerebras_rs::Client;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("your-api-key-here");
//!     // Now you can use client to make API calls
//!     Ok(())
//! }
//! ```
//! 
//! ## Chat Completions
//! 
//! ```rust,no_run
//! use cerebras_rs::{Client, ChatCompletionRequest, ChatMessage, ModelIdentifier};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::from_env()?;
//!     
//!     let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
//!         .system_message("You are a helpful assistant.")
//!         .user_message("What is the capital of France?")
//!         .max_tokens(100)
//!         .temperature(0.7)
//!         .build();
//!     
//!     let response = client.chat_completion(request).await?;
//!     if let Some(choices) = &response.choices {
//!         if let Some(choice) = choices.first() {
//!             if let Some(message) = &choice.message {
//!                 println!("{}", message.content);
//!             }
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Error Handling
//! 
//! ```rust,no_run
//! use cerebras_rs::{Client, ChatCompletionRequest, ChatMessage, ModelIdentifier, Error};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::from_env()?;
//!     let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
//!         .user_message("Hello")
//!         .build();
//!     
//!     match client.chat_completion(request).await {
//!         Ok(response) => println!("Success!"),
//!         Err(Error::RateLimit(retry_after)) => {
//!             println!("Rate limited, retry after {} seconds", retry_after);
//!         },
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Completions
//! 
//! ```rust,no_run
//! use cerebras_rs::{Client, CompletionRequest, ModelIdentifier, models::Prompt};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::from_env()?;
//!     
//!     let request = CompletionRequest::builder(ModelIdentifier::Llama3Period18b)
//!         .prompt("Once upon a time")
//!         .max_tokens(100)
//!         .temperature(0.7)
//!         .build();
//!     
//!     let response = client.completion(request).await?;
//!     if let Some(choices) = &response.choices {
//!         if let Some(choice) = choices.first() {
//!             if let Some(text) = &choice.text {
//!                 println!("{}", text);
//!             }
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Streaming
//! 
//! ```rust,no_run
//! use cerebras_rs::{Client, ChatCompletionRequest, ModelIdentifier};
//! use futures_util::StreamExt;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::from_env()?;
//!     
//!     let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
//!         .user_message("Tell me a story")
//!         .stream(true)
//!         .build();
//!     
//!     let mut stream = client.chat_completion_stream(request).await?;
//!     
//!     while let Some(chunk) = stream.next().await {
//!         match chunk {
//!             Ok(chunk) => {
//!                 if let Some(choices) = &chunk.choices {
//!                     if let Some(choice) = choices.first() {
//!                         if let Some(delta) = &choice.delta {
//!                             if let Some(content) = &delta.content {
//!                                 print!("{}", content);
//!                             }
//!                         }
//!                     }
//!                 }
//!             }
//!             Err(e) => eprintln!("Error: {}", e),
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Function Calling
//! 
//! ```rust,no_run
//! use cerebras_rs::{Client, ChatCompletionRequest, ChatMessage, ModelIdentifier};
//! use cerebras_rs::models::{Tool, FunctionDefinition, tool::Type, ToolChoiceOption};
//! use serde_json::json;
//! use std::collections::HashMap;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::from_env()?;
//!     
//!     // Define a weather function tool
//!     let weather_tool = Tool {
//!         r#type: Some(Type::Function),
//!         function: Some(FunctionDefinition {
//!             name: "get_weather".to_string(),
//!             description: Some("Get current weather".to_string()),
//!             parameters: Some(HashMap::new()), // Simplified for example
//!         }),
//!     };
//!     
//!     let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
//!         .user_message("What's the weather like in San Francisco?")
//!         .tool(weather_tool)
//!         .build();
//!     
//!     let response = client.chat_completion(request).await?;
//!     // Process response...
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Advanced: Custom Configuration
//! 
//! ```rust,no_run
//! use cerebras_rs::{Client, Configuration, ChatCompletionRequest, ModelIdentifier};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a custom configuration
//!     let mut config = Configuration::new();
//!     config.bearer_access_token = Some("your-api-key".to_string());
//!     config.base_path = "https://custom-endpoint.example.com".to_string();
//!     
//!     let client = Client::with_configuration(config);
//!     
//!     let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
//!         .user_message("Hello")
//!         .build();
//!     
//!     let response = client.chat_completion(request).await?;
//!     // Process response...
//!     
//!     Ok(())
//! }
//! ```

extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;
extern crate reqwest;
extern crate uuid;
extern crate chrono;
extern crate base64;

extern crate tokio;
extern crate async_trait;

extern crate tokio_stream;
extern crate futures_util;
extern crate eventsource_stream;
extern crate pin_project_lite;

extern crate thiserror;
extern crate anyhow;

pub mod apis;
pub mod models;

// Re-export commonly used types at the crate root
pub use apis::configuration::{ApiKey, Configuration};
pub use apis::default_api as api;

// Re-export all models at the crate root for convenience
pub use models::*;

// Re-export the main client
mod client;
pub use client::Client;

// Builder patterns
pub mod builders;

// Streaming support
pub mod streaming;

// Error handling
mod error;
pub use error::{Error, Result};

// Prelude module for convenient imports
pub mod prelude {
    //! The prelude module provides convenient imports for common usage.
    //! 
    //! # Example
    //! ```rust
    //! use cerebras_rs::prelude::*;
    //! ```
    
    pub use crate::{
        Client,
        Error,
        Result,
        ChatCompletionRequest,
        ChatMessage,
        CompletionRequest,
        ModelIdentifier,
    };
    
    pub use crate::streaming::{ChatCompletionStream, CompletionStream};
    
    pub use crate::builders::{ChatCompletionBuilder, CompletionBuilder};
}

// Version information
/// The version of the Cerebras Rust SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The OpenAPI specification version this SDK was generated from
pub const OPENAPI_VERSION: &str = "1.0.0";
