//! Example of using the Cerebras SDK for chat completions

use cerebras_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client from environment variable
    let client = Client::from_env()?;

    // Example 1: Simple chat completion
    println!("=== Simple Chat Completion ===");
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .system_message("You are a helpful assistant that provides concise answers.")
        .user_message("What is the capital of France?")
        .temperature(0.7)
        .max_tokens(100)
        .build();

    let response = client.chat_completion(request).await?;
    if let Some(choices) = &response.choices {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = &first_choice.message {
                println!("Response: {}", message.content);
            }
        }
    }

    // Example 2: Multi-turn conversation
    println!("\n=== Multi-turn Conversation ===");
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .system_message("You are a helpful math tutor.")
        .user_message("What is 15 + 27?")
        .assistant_message("15 + 27 = 42")
        .user_message("Great! Now what is 42 * 3?")
        .temperature(0.5)
        .build();

    let response = client.chat_completion(request).await?;
    if let Some(choices) = &response.choices {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = &first_choice.message {
                println!("Response: {}", message.content);
            }
        }
    }

    // Example 3: Using different models
    println!("\n=== Using Different Models ===");
    let models = vec![
        ModelIdentifier::Llama3Period18b,
        ModelIdentifier::Llama3Period370b,
    ];

    for model in models {
        println!("\nUsing model: {:?}", model);
        let request = ChatCompletionRequest::builder(model)
            .user_message("Write a haiku about programming")
            .temperature(0.9)
            .max_tokens(50)
            .build();

        let response = client.chat_completion(request).await?;
        if let Some(choices) = &response.choices {
            if let Some(first_choice) = choices.first() {
                if let Some(message) = &first_choice.message {
                    println!("{}", message.content);
                }
            }
        }
    }

    // Example 4: JSON response format
    println!("\n=== JSON Response Format ===");
    // First example - just asking for JSON in the prompt
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("List 3 programming languages with their main use cases in JSON format")
        .temperature(0.3)
        .build();

    let response = client.chat_completion(request).await?;
    if let Some(choices) = &response.choices {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = &first_choice.message {
                println!("JSON Response: {}", message.content);
            }
        }
    }

    // Example 4b: Using JSON schema (commented out as it requires API support)
    // println!("\n=== JSON Schema Response Format ===");
    // let schema = serde_json::json!({
    //     "type": "object",
    //     "properties": {
    //         "languages": {
    //             "type": "array",
    //             "items": {
    //                 "type": "object",
    //                 "properties": {
    //                     "name": { "type": "string" },
    //                     "use_cases": {
    //                         "type": "array",
    //                         "items": { "type": "string" }
    //                     }
    //                 },
    //                 "required": ["name", "use_cases"]
    //             }
    //         }
    //     },
    //     "required": ["languages"]
    // });
    //
    // let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
    //     .user_message("List 3 programming languages with their main use cases")
    //     .json_schema("programming_languages", schema, true)
    //     .temperature(0.3)
    //     .build();
    //
    // let response = client.chat_completion(request).await?;
    // if let Some(choices) = &response.choices {
    //     if let Some(first_choice) = choices.first() {
    //         if let Some(message) = &first_choice.message {
    //             println!("Structured JSON Response: {}", message.content);
    //         }
    //     }
    // }

    // Example 5: Using stop sequences
    println!("\n=== Using Stop Sequences ===");
    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Count from 1 to 10:")
        .stop_sequence("5")
        .temperature(0.1)
        .build();

    let response = client.chat_completion(request).await?;
    if let Some(choices) = &response.choices {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = &first_choice.message {
                println!("Response (stops at 5): {}", message.content);
            }
        }
    }

    // Print usage information
    if let Some(usage) = &response.usage {
        println!("\nToken usage:");
        if let Some(prompt_tokens) = usage.prompt_tokens {
            println!("  Prompt tokens: {}", prompt_tokens);
        }
        if let Some(completion_tokens) = usage.completion_tokens {
            println!("  Completion tokens: {}", completion_tokens);
        }
        if let Some(total_tokens) = usage.total_tokens {
            println!("  Total tokens: {}", total_tokens);
        }
    }

    // Print timing information
    if let Some(time_info) = &response.time_info {
        println!("\nTiming information:");
        if let Some(queue_time) = time_info.queue_time {
            println!("  Queue time: {:.3}s", queue_time);
        }
        if let Some(prompt_time) = time_info.prompt_time {
            println!("  Prompt time: {:.3}s", prompt_time);
        }
        if let Some(completion_time) = time_info.completion_time {
            println!("  Completion time: {:.3}s", completion_time);
        }
        if let Some(total_time) = time_info.total_time {
            println!("  Total time: {:.3}s", total_time);
        }
    }

    Ok(())
}
