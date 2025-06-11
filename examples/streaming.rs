//! Example of using streaming responses with the Cerebras SDK

use cerebras_rs::prelude::*;
use futures_util::StreamExt;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client
    let client = Client::from_env()?;

    // Example 1: Basic streaming chat completion
    println!("=== Streaming Chat Completion ===");
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Tell me a short story about a robot learning to paint.")
        .temperature(0.8)
        .max_tokens(200)
        .stream(true)
        .build();

    let mut stream = client.chat_completion_stream(request).await?;

    print!("Story: ");
    let _ = io::stdout().flush();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                if let Some(choices) = &chunk.choices {
                    for choice in choices {
                        if let Some(delta) = &choice.delta {
                            if let Some(content) = &delta.content {
                                print!("{}", content);
                                let _ = io::stdout().flush();
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("\nError: {}", e),
        }
    }
    println!("\n");

    // Example 2: Streaming with progress indicator
    println!("=== Streaming with Progress ===");
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Explain quantum computing in simple terms.")
        .temperature(0.7)
        .max_tokens(150)
        .stream(true)
        .build();

    let mut stream = client.chat_completion_stream(request).await?;
    let mut token_count = 0;

    print!("Response: ");
    let _ = io::stdout().flush();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                if let Some(choices) = &chunk.choices {
                    for choice in choices {
                        if let Some(delta) = &choice.delta {
                            if let Some(content) = &delta.content {
                                print!("{}", content);
                                let _ = io::stdout().flush();
                                token_count += 1;
                            }
                        }

                        if let Some(reason) = &choice.finish_reason {
                            println!("\n\nFinished: {:?}", reason);
                            println!("Approximate tokens generated: {}", token_count);
                        }
                    }
                }
            }
            Err(e) => eprintln!("\nError: {}", e),
        }
    }

    // Example 3: Collecting stream into complete response
    println!("\n=== Collecting Stream ===");
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("What are the three laws of robotics?")
        .temperature(0.5)
        .stream(true)
        .build();

    println!("Collecting response...");
    let stream = client.chat_completion_stream(request).await?;
    let complete_response = stream.collect().await?;

    if let Some(choices) = &complete_response.choices {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = &first_choice.message {
                println!("Complete response collected:");
                println!("{}", message.content);
            }
        }
    }

    // Example 4: Streaming text completion
    println!("\n=== Streaming Text Completion ===");
    let request = CompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .prompt("The future of artificial intelligence is")
        .temperature(0.7)
        .max_tokens(100)
        .stream(true)
        .build();

    let mut stream = client.completion_stream(request).await?;

    print!("Completion: The future of artificial intelligence is");
    let _ = io::stdout().flush();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                if let Some(choices) = &chunk.choices {
                    for choice in choices {
                        if let Some(text) = &choice.text {
                            print!("{}", text);
                            let _ = io::stdout().flush();
                        }
                    }
                }
            }
            Err(e) => eprintln!("\nError: {}", e),
        }
    }
    println!("\n");

    // Example 5: Handling errors in streaming
    println!("=== Error Handling in Streaming ===");
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Test message")
        .temperature(0.5)
        .max_tokens(10000) // Intentionally high to potentially trigger limits
        .stream(true)
        .build();

    let mut stream = client.chat_completion_stream(request).await?;
    let mut error_count = 0;

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                if let Some(choices) = &chunk.choices {
                    for choice in choices {
                        if let Some(delta) = &choice.delta {
                            if let Some(content) = &delta.content {
                                print!("{}", content);
                                let _ = io::stdout().flush();
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                eprintln!("\nStreaming error #{}: {}", error_count, e);

                // Decide whether to continue or break based on error type
                match &e {
                    Error::RateLimit(retry_after) => {
                        eprintln!("Rate limit hit, retry after {} seconds", retry_after);
                        break;
                    }
                    _ => {
                        eprintln!("Continuing despite error");
                    }
                }
            }
        }
    }

    if error_count > 0 {
        println!("\nEncountered {} errors during streaming", error_count);
    }

    Ok(())
}
