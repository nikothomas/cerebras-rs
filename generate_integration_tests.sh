#!/bin/bash
set -e

# Change to the project root directory
cd "$(dirname "$0")"

echo "Generating integration tests..."

OUTPUT_FILE="tests/integration.rs"
TEMP_FILE=$(mktemp)

# Get the list of public methods from the Client struct
CLIENT_METHODS=$(grep -o 'pub fn [a-z_][a-zA-Z0-9_]*' src/client.rs | awk '{print $3}' | sort -u)

# Initialize the test file with header
cat > "$TEMP_FILE" << 'EOF'
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
        // Try to get API key from environment, fall back to "test_key" for CI
        let api_key = env::var("CEREBRAS_API_KEY").unwrap_or_else(|_| "test_key".to_string());
        Client::new(&api_key)
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
            prompt: Some("Hello, world!".to_string()),
            ..Default::default()
        }
    }
EOF

# Function to generate test for a Client method
generate_test() {
    local method=$1
    local test_name="test_${method}"
    
    echo "    #[tokio::test]" >> "$TEMP_FILE"
    echo "    async fn ${test_name}() {" >> "$TEMP_FILE"
    
    case $method in
        "new")
            echo "        let client = create_test_client();" >> "$TEMP_FILE"
            echo "        assert!(!client.configuration().base_path.is_empty());" >> "$TEMP_FILE"
            ;;
            
        "from_env")
            echo "        env::set_var(\"CEREBRAS_API_KEY\", \"test_key\");" >> "$TEMP_FILE"
            echo "        let client = Client::from_env().expect(\"Failed to create client from env\");" >> "$TEMP_FILE"
            echo "        assert!(!client.configuration().base_path.is_empty());" >> "$TEMP_FILE"
            ;;
            
        "with_configuration")
            echo "        let config = Configuration::new();" >> "$TEMP_FILE"
            echo "        let client = Client::with_configuration(config);" >> "$TEMP_FILE"
            echo "        assert!(!client.configuration().base_path.is_empty());" >> "$TEMP_FILE"
            ;;
            
        "with_base_url")
            echo "        let client = create_test_client()" >> "$TEMP_FILE"
            echo "            .with_base_url(\"http://example.com\".to_string());" >> "$TEMP_FILE"
            echo "        assert_eq!(client.configuration().base_path, \"http://example.com\");" >> "$TEMP_FILE"
            ;;
            
        "list_models")
            echo "        if !requires_api_key() { return; }" >> "$TEMP_FILE"
            echo "        let client = create_test_client();" >> "$TEMP_FILE"
            echo "        let result = client.list_models().await;" >> "$TEMP_FILE"
            echo "        assert!(result.is_ok(), \"list_models failed: {:?}\", result);" >> "$TEMP_FILE"
            ;;
            
        "get_model")
            echo "        if !requires_api_key() { return; }" >> "$TEMP_FILE"
            echo "        let client = create_test_client();" >> "$TEMP_FILE"
            echo "        let model_id = ModelIdentifier::Llama3Period18b;" >> "$TEMP_FILE"
            echo "        let result = client.get_model(model_id).await;" >> "$TEMP_FILE"
            echo "        assert!(result.is_ok(), \"get_model failed: {:?}\", result);" >> "$TEMP_FILE"
            ;;
            
        "chat_completion")
            echo "        if !requires_api_key() { return; }" >> "$TEMP_FILE"
            echo "        let client = create_test_client();" >> "$TEMP_FILE"
            echo "        let request = create_test_chat_request();" >> "$TEMP_FILE"
            echo "        let result = client.chat_completion(request).await;" >> "$TEMP_FILE"
            echo "        assert!(result.is_ok(), \"chat_completion failed: {:?}\", result);" >> "$TEMP_FILE"
            ;;
            
        "chat_completion_stream")
            echo "        if !requires_api_key() { return; }" >> "$TEMP_FILE"
            echo "        let client = create_test_client();" >> "$TEMP_FILE"
            echo "        let mut request = create_test_chat_request();" >> "$TEMP_FILE"
            echo "        request.stream = Some(true);" >> "$TEMP_FILE"
            echo "        let mut stream = client.chat_completion_stream(request).await;" >> "$TEMP_FILE"
            echo "        let first_chunk = stream.next().await;" >> "$TEMP_FILE"
            echo "        assert!(first_chunk.is_some(), \"Expected at least one chunk in stream\");" >> "$TEMP_FILE"
            ;;
            
        "completion")
            echo "        if !requires_api_key() { return; }" >> "$TEMP_FILE"
            echo "        let client = create_test_client();" >> "$TEMP_FILE"
            echo "        let request = create_test_completion_request();" >> "$TEMP_FILE"
            echo "        let result = client.completion(request).await;" >> "$TEMP_FILE"
            echo "        assert!(result.is_ok(), \"completion failed: {:?}\", result);" >> "$TEMP_FILE"
            ;;
            
        "completion_stream")
            echo "        if !requires_api_key() { return; }" >> "$TEMP_FILE"
            echo "        let client = create_test_client();" >> "$TEMP_FILE"
            echo "        let mut request = create_test_completion_request();" >> "$TEMP_FILE"
            echo "        request.stream = Some(true);" >> "$TEMP_FILE"
            echo "        let mut stream = client.completion_stream(request).await;" >> "$TEMP_FILE"
            echo "        let first_chunk = stream.next().await;" >> "$TEMP_FILE"
            echo "        assert!(first_chunk.is_some(), \"Expected at least one chunk in stream\");" >> "$TEMP_FILE"
            ;;
            
        *)
            # Skip test generation for unrecognized method: $method
            ;;
    esac
    
    echo "    }" >> "$TEMP_FILE"
    echo "" >> "$TEMP_FILE"
}

# Generate tests for all public methods in Client
for method in $CLIENT_METHODS; do
    # Skip private methods and trait implementations
    if [[ ! "$method" =~ ^_ ]] && [[ ! "$method" =~ ^(clone|drop|eq|fmt|hash|partial_eq|partial_ord|ord)$ ]]; then
        echo "Generating test for method: $method"
        generate_test "$method"
    fi
done

# Close the test module
echo "}" >> "$TEMP_FILE"

# Add any additional test helpers
cat >> "$TEMP_FILE" << 'EOF'

// Add any additional test helpers below
// For example, test utilities or shared test data

#[cfg(test)]
mod test_helpers {
    // Test helper functions can go here
}
EOF

mv "$TEMP_FILE" "$OUTPUT_FILE"
echo "Integration tests generated successfully in ${OUTPUT_FILE}."
echo ""
echo "To run the tests:"
echo "1. Set your API key: export CEREBRAS_API_KEY='your-api-key'"
echo "2. Run: cargo test --test integration -- --nocapture"
