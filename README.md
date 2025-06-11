# Cerebras Rust SDK

[![Crates.io](https://img.shields.io/crates/v/cerebras-rs.svg)](https://crates.io/crates/cerebras-rs)
[![Documentation](https://docs.rs/cerebras-rs/badge.svg)](https://docs.rs/cerebras-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance Rust SDK for the Cerebras Inference API, providing low-latency AI model inference powered by Cerebras Wafer-Scale Engines and CS-3 systems.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cerebras-rs = "0.0.1"
```

## Quick Start

```rust,no_run
use cerebras_rs::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable CEREBRAS_API_KEY
    let client = Client::from_env()?;
    
    // Create a chat completion
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .system_message("You are a helpful assistant")
        .user_message("What is the capital of France?")
        .temperature(0.7)
        .build();
    
    let response = client.chat_completion(request).await?;
    if let Some(choices) = &response.choices {
        if let Some(choice) = choices.first() {
            if let Some(message) = &choice.message {
                println!("{}", message.content);
            }
        }
    }
    
    Ok(())
}
```

## Authentication

Set your API key as an environment variable:

```bash
export CEREBRAS_API_KEY="your-api-key-here"
```

Or create a client with an explicit API key:

```rust
use cerebras_rs::Client;

let client = Client::new("your-api-key-here");
```

## Available Models

The SDK supports all Cerebras models:

- `ModelIdentifier::Llama4Scout17b16eInstruct` - Llama 4 Scout 17B
- `ModelIdentifier::Llama3Period18b` - Llama 3.1 8B
- `ModelIdentifier::Llama3Period370b` - Llama 3.3 70B
- `ModelIdentifier::Qwen332b` - Qwen 3 32B
- `ModelIdentifier::DeepseekR1DistillLlama70b` - Deepseek R1 Distill Llama 70B

## Examples

### Chat Completion with Builder Pattern

```rust,no_run
use cerebras_rs::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .system_message("You are a helpful math tutor")
        .user_message("What is 15 + 27?")
        .temperature(0.3)
        .max_tokens(100)
        .build();

    let response = client.chat_completion(request).await?;
    if let Some(choices) = &response.choices {
        if let Some(choice) = choices.first() {
            if let Some(message) = &choice.message {
                println!("{}", message.content);
            }
        }
    }
    
    Ok(())
}
```

### Streaming Responses

```rust,no_run
use cerebras_rs::prelude::*;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Tell me a story")
        .stream(true)
        .build();

    let mut stream = client.chat_completion_stream(request).await?;

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                if let Some(choices) = &chunk.choices {
                    if let Some(choice) = choices.first() {
                        if let Some(delta) = &choice.delta {
                            if let Some(content) = &delta.content {
                                print!("{}", content);
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}
```

### Function Calling

```rust,no_run
use cerebras_rs::prelude::*;
use cerebras_rs::models::{Tool, FunctionDefinition, tool::Type};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let weather_tool = Tool {
        r#type: Some(Type::Function),
        function: Some(FunctionDefinition {
            name: "get_weather".to_string(),
            description: Some("Get current weather".to_string()),
            parameters: Some(HashMap::new()), // Simplified for example
        }),
    };

    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("What's the weather in New York?")
        .tool(weather_tool)
        .build();

    let response = client.chat_completion(request).await?;

    if let Some(choices) = &response.choices {
        if let Some(choice) = choices.first() {
            if let Some(message) = &choice.message {
                if let Some(tool_calls) = &message.tool_calls {
                    for call in tool_calls {
                        println!("Function: {}", call.name.as_ref().unwrap_or(&"".to_string()));
                        println!("Arguments: {}", call.arguments.as_ref().unwrap_or(&"".to_string()));
                    }
                }
            }
        }
    }
    
    Ok(())
}
```

### Text Completion

```rust,no_run
use cerebras_rs::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    
    let request = CompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .prompt("Once upon a time")
        .max_tokens(100)
        .temperature(0.8)
        .build();

    let response = client.completion(request).await?;
    if let Some(choices) = &response.choices {
        if let Some(choice) = choices.first() {
            if let Some(text) = &choice.text {
                println!("{}", text);
            }
        }
    }
    
    Ok(())
}
```

## Error Handling

The SDK provides comprehensive error handling:

```rust,no_run
use cerebras_rs::{Client, ChatCompletionRequest, ModelIdentifier, Error};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Hello")
        .build();
    
    match client.chat_completion(request).await {
        Ok(response) => {
            if let Some(choices) = &response.choices {
                if let Some(choice) = choices.first() {
                    if let Some(message) = &choice.message {
                        println!("Success: {}", message.content);
                    }
                }
            }
        }
        Err(Error::RateLimit(retry_after)) => {
            println!("Rate limited. Retry after {} seconds", retry_after);
        }
        Err(Error::Authentication) => {
            println!("Invalid API key");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    Ok(())
}
```

## Advanced Features

### Custom Configuration

```rust,no_run
use cerebras_rs::{Client, Configuration, ChatCompletionRequest, ModelIdentifier};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new();
    config.base_path = "https://custom-endpoint.example.com".to_string();
    config.bearer_access_token = Some("your-api-key".to_string());

    let client = Client::with_configuration(config);
    
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Hello")
        .build();
    
    let response = client.chat_completion(request).await?;
    // Process response...
    
    Ok(())
}
```

### Response Metadata

```rust,no_run
use cerebras_rs::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env()?;
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Hello")
        .build();
    
    let response = client.chat_completion(request).await?;

    // Token usage
    if let Some(usage) = &response.usage {
        println!("Prompt tokens: {}", usage.prompt_tokens.unwrap_or(0));
        println!("Completion tokens: {}", usage.completion_tokens.unwrap_or(0));
        println!("Total tokens: {}", usage.total_tokens.unwrap_or(0));
    }

    // Timing information
    if let Some(time_info) = &response.time_info {
        println!("Queue time: {:.3}s", time_info.queue_time.unwrap_or(0.0));
        println!("Prompt time: {:.3}s", time_info.prompt_time.unwrap_or(0.0));
        println!("Completion time: {:.3}s", time_info.completion_time.unwrap_or(0.0));
        println!("Total time: {:.3}s", time_info.total_time.unwrap_or(0.0));
    }
    
    Ok(())
}
```

## More Examples

Check out the `examples/` directory for more comprehensive examples:

- `chat_completion.rs` - Various chat completion scenarios
- `streaming.rs` - Streaming response handling
- `function_calling.rs` - Tool use and function calling

Run examples with:

```bash
cargo run --example chat_completion
cargo run --example streaming
cargo run --example function_calling
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Links

- [API Documentation](https://docs.rs/cerebras-rs)
- [Cerebras Inference Docs](https://inference-docs.cerebras.ai)
- [GitHub Repository](https://github.com/cerebras/cerebras-rs)
