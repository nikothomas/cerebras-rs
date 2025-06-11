//! Builder patterns for constructing API requests
//!
//! This module provides ergonomic builder patterns for creating various request types.

mod chat_completion_builder;
mod completion_builder;

pub use chat_completion_builder::ChatCompletionBuilder;
pub use completion_builder::CompletionBuilder;
