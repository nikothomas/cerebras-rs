//! Error types for the Cerebras Rust SDK

use thiserror::Error;

/// Result type alias for Cerebras SDK operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the Cerebras SDK
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    /// API returned an error response
    #[error("API error: {0}")]
    Api(String),
    
    /// API returned an error with details
    #[error("API error ({code}): {message}")]
    ApiError {
        /// Error type
        error_type: String,
        /// Error code
        code: String,
        /// Error message
        message: String,
        /// Parameter that caused the error (if applicable)
        param: Option<String>,
    },
    
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// Invalid model identifier
    #[error("Invalid model identifier: {0}")]
    InvalidModel(String),
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded. Please retry after {0} seconds")]
    RateLimit(u64),
    
    /// Authentication failed
    #[error("Authentication failed. Please check your API key")]
    Authentication,
    
    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    /// Invalid request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    /// Server error
    #[error("Server error: {0}")]
    ServerError(String),
    
    /// Streaming error
    #[error("Streaming error: {0}")]
    Streaming(String),
    
    /// Timeout error
    #[error("Request timed out")]
    Timeout,
    
    /// Unknown error
    #[error("Unknown error occurred")]
    Unknown,
}

impl From<crate::apis::Error<crate::apis::default_api::CreateChatCompletionError>> for Error {
    fn from(err: crate::apis::Error<crate::apis::default_api::CreateChatCompletionError>) -> Self {
        use crate::apis::Error as ApiError;
        use crate::apis::default_api::CreateChatCompletionError;
        
        match err {
            ApiError::Reqwest(e) => Error::Http(e),
            ApiError::Serde(e) => Error::Serialization(e),
            ApiError::Io(e) => Error::Api(format!("IO error: {}", e)),
            ApiError::ResponseError(response) => {
                match response.entity {
                    Some(CreateChatCompletionError::Status400(detail)) => {
                        Error::InvalidRequest(detail.message.unwrap_or_else(|| "Bad request".to_string()))
                    }
                    Some(CreateChatCompletionError::Status401(_)) => {
                        Error::Authentication
                    }
                    Some(CreateChatCompletionError::Status422(detail)) => {
                        Error::InvalidRequest(detail.message.unwrap_or_else(|| "Invalid parameters".to_string()))
                    }
                    Some(CreateChatCompletionError::Status429(_detail)) => {
                        Error::RateLimit(0) // Could parse from headers if available
                    }
                    Some(CreateChatCompletionError::Status500(detail)) => {
                        Error::ServerError(detail.message.unwrap_or_else(|| "Internal server error".to_string()))
                    }
                    _ => Error::Api(format!("HTTP {}: {}", response.status, response.content)),
                }
            }
        }
    }
}

impl From<crate::apis::Error<crate::apis::default_api::CreateCompletionError>> for Error {
    fn from(err: crate::apis::Error<crate::apis::default_api::CreateCompletionError>) -> Self {
        use crate::apis::Error as ApiError;
        use crate::apis::default_api::CreateCompletionError;
        
        match err {
            ApiError::Reqwest(e) => Error::Http(e),
            ApiError::Serde(e) => Error::Serialization(e),
            ApiError::Io(e) => Error::Api(format!("IO error: {}", e)),
            ApiError::ResponseError(response) => {
                match response.entity {
                    Some(CreateCompletionError::Status400(detail)) => {
                        Error::InvalidRequest(detail.message.unwrap_or_else(|| "Bad request".to_string()))
                    }
                    Some(CreateCompletionError::Status401(_)) => {
                        Error::Authentication
                    }
                    Some(CreateCompletionError::Status422(detail)) => {
                        Error::InvalidRequest(detail.message.unwrap_or_else(|| "Invalid parameters".to_string()))
                    }
                    Some(CreateCompletionError::Status429(_detail)) => {
                        Error::RateLimit(0)
                    }
                    Some(CreateCompletionError::Status500(detail)) => {
                        Error::ServerError(detail.message.unwrap_or_else(|| "Internal server error".to_string()))
                    }
                    _ => Error::Api(format!("HTTP {}: {}", response.status, response.content)),
                }
            }
        }
    }
}

impl From<crate::apis::Error<crate::apis::default_api::ListModelsError>> for Error {
    fn from(err: crate::apis::Error<crate::apis::default_api::ListModelsError>) -> Self {
        use crate::apis::Error as ApiError;
        use crate::apis::default_api::ListModelsError;
        
        match err {
            ApiError::Reqwest(e) => Error::Http(e),
            ApiError::Serde(e) => Error::Serialization(e),
            ApiError::Io(e) => Error::Api(format!("IO error: {}", e)),
            ApiError::ResponseError(response) => {
                match response.entity {
                    Some(ListModelsError::Status401(_)) => Error::Authentication,
                    Some(ListModelsError::Status429(_)) => Error::RateLimit(0),
                    Some(ListModelsError::Status500(detail)) => {
                        Error::ServerError(detail.message.unwrap_or_else(|| "Internal server error".to_string()))
                    }
                    _ => Error::Api(format!("HTTP {}: {}", response.status, response.content)),
                }
            }
        }
    }
}

impl From<crate::apis::Error<crate::apis::default_api::RetrieveModelError>> for Error {
    fn from(err: crate::apis::Error<crate::apis::default_api::RetrieveModelError>) -> Self {
        use crate::apis::Error as ApiError;
        use crate::apis::default_api::RetrieveModelError;
        
        match err {
            ApiError::Reqwest(e) => Error::Http(e),
            ApiError::Serde(e) => Error::Serialization(e),
            ApiError::Io(e) => Error::Api(format!("IO error: {}", e)),
            ApiError::ResponseError(response) => {
                match response.entity {
                    Some(RetrieveModelError::Status401(_)) => Error::Authentication,
                    Some(RetrieveModelError::Status404(detail)) => {
                        Error::NotFound(detail.message.unwrap_or_else(|| "Model not found".to_string()))
                    }
                    Some(RetrieveModelError::Status429(_)) => Error::RateLimit(0),
                    Some(RetrieveModelError::Status500(detail)) => {
                        Error::ServerError(detail.message.unwrap_or_else(|| "Internal server error".to_string()))
                    }
                    _ => Error::Api(format!("HTTP {}: {}", response.status, response.content)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let err = Error::Authentication;
        assert_eq!(err.to_string(), "Authentication failed. Please check your API key");
        
        let err = Error::RateLimit(60);
        assert_eq!(err.to_string(), "Rate limit exceeded. Please retry after 60 seconds");
    }
}
