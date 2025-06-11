//! Builder pattern for CompletionRequest

use crate::models::{CompletionRequest, ModelIdentifier, Prompt, StopCondition};

/// Builder for creating CompletionRequest instances
///
/// # Example
/// ```rust,no_run
/// use cerebras_rs::builders::CompletionBuilder;
/// use cerebras_rs::ModelIdentifier;
///
/// let request = CompletionBuilder::new(ModelIdentifier::Llama3Period18b)
///     .prompt("Once upon a time")
///     .max_tokens(100)
///     .temperature(0.8)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct CompletionBuilder {
    model: ModelIdentifier,
    prompt: Option<Prompt>,
    max_tokens: Option<u32>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    stream: Option<bool>,
    stop: Option<Vec<String>>,
    return_raw_tokens: Option<bool>,
}

impl CompletionBuilder {
    /// Create a new builder with the specified model
    pub fn new(model: ModelIdentifier) -> Self {
        Self {
            model,
            prompt: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            stream: None,
            stop: None,
            return_raw_tokens: None,
        }
    }

    /// Set the prompt as a string
    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(Prompt::String(prompt.into()));
        self
    }

    /// Set multiple prompts
    pub fn prompts(mut self, prompts: Vec<String>) -> Self {
        // Since the API might not support array prompts, join them
        self.prompt = Some(Prompt::String(prompts.join("\n")));
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

    /// Return raw tokens instead of text
    pub fn return_raw_tokens(mut self, return_raw: bool) -> Self {
        self.return_raw_tokens = Some(return_raw);
        self
    }

    /// Build the CompletionRequest
    pub fn build(self) -> CompletionRequest {
        CompletionRequest {
            model: self.model,
            prompt: self.prompt.unwrap_or(Prompt::String("".to_string())),
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
            return_raw_tokens: self.return_raw_tokens,
        }
    }
}

impl CompletionRequest {
    /// Create a new builder for this request type
    pub fn builder(model: ModelIdentifier) -> CompletionBuilder {
        CompletionBuilder::new(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let request = CompletionBuilder::new(ModelIdentifier::Llama3Period18b)
            .prompt("Hello world")
            .temperature(0.5)
            .max_tokens(50)
            .build();

        match request.prompt {
            Prompt::String(s) => assert_eq!(s, "Hello world"),
            _ => panic!("Expected string prompt"),
        }
        assert_eq!(request.temperature, Some(0.5));
        assert_eq!(request.max_tokens, Some(50));
    }

    #[test]
    fn test_builder_multiple_prompts() {
        let request = CompletionBuilder::new(ModelIdentifier::Llama3Period18b)
            .prompts(vec!["First".to_string(), "Second".to_string()])
            .build();

        match request.prompt {
            Prompt::String(s) => {
                assert_eq!(s, "First\nSecond");
            }
            _ => panic!("Expected string prompt"),
        }
    }
}
