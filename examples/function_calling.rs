//! Example of function calling with the Cerebras SDK

use cerebras_rs::models::{FunctionDefinition, FunctionName, Tool, ToolChoiceOption, tool};
use cerebras_rs::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client
    let client = Client::from_env()?;

    // Example 1: Basic function calling
    println!("=== Basic Function Calling ===");

    // Define a weather function
    let weather_function = Tool {
        r#type: Some(tool::Type::Function),
        function: Some(FunctionDefinition::new("get_weather".to_string())),
    };

    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("What's the weather like in New York?")
        .tool(weather_function)
        .tool_choice(ToolChoiceOption::String("auto".to_string()))
        .temperature(0.3)
        .build();

    let response = client.chat_completion(request).await?;

    // Check if the model wants to call a function
    if let Some(choices) = &response.choices {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = &first_choice.message {
                if let Some(tool_calls) = &message.tool_calls {
                    for tool_call in tool_calls {
                        println!(
                            "Function call: {}",
                            tool_call.name.as_ref().unwrap_or(&"unknown".to_string())
                        );
                        println!(
                            "Arguments: {}",
                            tool_call.arguments.as_ref().unwrap_or(&"{}".to_string())
                        );

                        // Simulate function execution
                        let weather_result =
                            simulate_weather_api(tool_call.arguments.as_deref().unwrap_or("{}"));

                        // Send the function result back to the model
                        let follow_up =
                            ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
                                .message(message.clone())
                                .message(ChatMessage::tool(
                                    weather_result,
                                    format!(
                                        "call_{}",
                                        tool_call.name.as_ref().unwrap_or(&"unknown".to_string())
                                    ),
                                ))
                                .temperature(0.3)
                                .build();

                        let final_response = client.chat_completion(follow_up).await?;
                        if let Some(final_choices) = &final_response.choices {
                            if let Some(final_choice) = final_choices.first() {
                                if let Some(final_message) = &final_choice.message {
                                    println!("Final response: {}", final_message.content);
                                }
                            }
                        }
                    }
                } else {
                    println!("Response: {}", message.content);
                }
            }
        }
    }

    // Example 2: Multiple functions
    println!("\n=== Multiple Functions ===");

    let calculator_function = Tool {
        r#type: Some(tool::Type::Function),
        function: Some(FunctionDefinition::new("calculate".to_string())),
    };

    let search_function = Tool {
        r#type: Some(tool::Type::Function),
        function: Some(FunctionDefinition::new("search_web".to_string())),
    };

    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("What is 42 * 17, and can you search for information about the result?")
        .tools(vec![calculator_function, search_function])
        .temperature(0.3)
        .build();

    let response = client.chat_completion(request).await?;

    if let Some(choices) = &response.choices {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = &first_choice.message {
                if let Some(tool_calls) = &message.tool_calls {
                    let mut messages = vec![message.clone()];

                    for tool_call in tool_calls {
                        println!(
                            "\nFunction: {}",
                            tool_call.name.as_ref().unwrap_or(&"unknown".to_string())
                        );
                        println!(
                            "Arguments: {}",
                            tool_call.arguments.as_ref().unwrap_or(&"{}".to_string())
                        );

                        let result = match tool_call.name.as_deref().unwrap_or("") {
                            "calculate" => {
                                simulate_calculator(tool_call.arguments.as_deref().unwrap_or("{}"))
                            }
                            "search_web" => {
                                simulate_web_search(tool_call.arguments.as_deref().unwrap_or("{}"))
                            }
                            _ => "Unknown function".to_string(),
                        };

                        messages.push(ChatMessage::tool(
                            result,
                            format!(
                                "call_{}",
                                tool_call.name.as_ref().unwrap_or(&"unknown".to_string())
                            ),
                        ));
                    }

                    // Get final response with all function results
                    let follow_up =
                        ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
                            .messages(messages)
                            .temperature(0.3)
                            .build();

                    let final_response = client.chat_completion(follow_up).await?;
                    if let Some(final_choices) = &final_response.choices {
                        if let Some(final_choice) = final_choices.first() {
                            if let Some(final_message) = &final_choice.message {
                                println!("\nFinal response: {}", final_message.content);
                            }
                        }
                    }
                }
            }
        }
    }

    // Example 3: Forcing specific function use
    println!("\n=== Forcing Function Use ===");

    let weather_function = Tool {
        r#type: Some(tool::Type::Function),
        function: Some(FunctionDefinition::new("get_weather".to_string())),
    };

    let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
        .user_message("Tell me about Paris")
        .tool(weather_function.clone())
        .tool_choice(ToolChoiceOption::FunctionName(FunctionName {
            name: Some("get_weather".to_string()),
        }))
        .temperature(0.3)
        .build();

    let response = client.chat_completion(request).await?;

    if let Some(choices) = &response.choices {
        if let Some(first_choice) = choices.first() {
            if let Some(message) = &first_choice.message {
                if let Some(tool_calls) = &message.tool_calls {
                    if let Some(first_call) = tool_calls.first() {
                        println!(
                            "Forced function call: {}",
                            first_call.name.as_ref().unwrap_or(&"unknown".to_string())
                        );
                        println!(
                            "Arguments: {}",
                            first_call.arguments.as_ref().unwrap_or(&"{}".to_string())
                        );
                    }
                }
            }
        }
    }

    // Example 4: Streaming with function calls
    {
        println!("\n=== Streaming with Functions ===");

        let calculator_function = Tool {
            r#type: Some(tool::Type::Function),
            function: Some(FunctionDefinition::new("calculate".to_string())),
        };

        let weather_function = Tool {
            r#type: Some(tool::Type::Function),
            function: Some(FunctionDefinition::new("get_weather".to_string())),
        };

        let request = ChatCompletionRequest::builder(ModelIdentifier::Llama3Period18b)
            .user_message("What's 15 + 27 and what's the weather in London?")
            .tools(vec![calculator_function.clone(), weather_function.clone()])
            .stream(true)
            .temperature(0.3)
            .build();

        let stream = client.chat_completion_stream(request).await?;
        let complete_response = stream.collect().await?;

        if let Some(choices) = &complete_response.choices {
            if let Some(first_choice) = choices.first() {
                if let Some(message) = &first_choice.message {
                    if let Some(tool_calls) = &message.tool_calls {
                        println!("Functions to call:");
                        for tool_call in tool_calls {
                            println!(
                                "  - {}: {}",
                                tool_call.name.as_ref().unwrap_or(&"unknown".to_string()),
                                tool_call.arguments.as_ref().unwrap_or(&"{}".to_string())
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

// Simulate API calls
fn simulate_weather_api(args: &str) -> String {
    let parsed: serde_json::Value = serde_json::from_str(args).unwrap_or(json!({}));
    let location = parsed["location"].as_str().unwrap_or("Unknown");
    let unit = parsed["unit"].as_str().unwrap_or("fahrenheit");

    json!({
        "location": location,
        "temperature": 72,
        "unit": unit,
        "conditions": "Partly cloudy",
        "humidity": 65,
        "wind_speed": 10
    })
    .to_string()
}

fn simulate_calculator(args: &str) -> String {
    let parsed: serde_json::Value = serde_json::from_str(args).unwrap_or(json!({}));
    let operation = parsed["operation"].as_str().unwrap_or("add");
    let a = parsed["a"].as_f64().unwrap_or(0.0);
    let b = parsed["b"].as_f64().unwrap_or(0.0);

    let result = match operation {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" => {
            if b != 0.0 {
                a / b
            } else {
                f64::NAN
            }
        }
        _ => 0.0,
    };

    json!({
        "result": result,
        "operation": operation,
        "a": a,
        "b": b
    })
    .to_string()
}

fn simulate_web_search(args: &str) -> String {
    let parsed: serde_json::Value = serde_json::from_str(args).unwrap_or(json!({}));
    let query = parsed["query"].as_str().unwrap_or("Unknown");
    let num_results = parsed["num_results"].as_u64().unwrap_or(5);

    json!({
        "query": query,
        "results": [
            {
                "title": format!("Result 1 for: {}", query),
                "url": "https://example.com/1",
                "snippet": "This is a simulated search result..."
            },
            {
                "title": format!("Result 2 for: {}", query),
                "url": "https://example.com/2",
                "snippet": "Another simulated result..."
            }
        ],
        "total_results": num_results
    })
    .to_string()
}
