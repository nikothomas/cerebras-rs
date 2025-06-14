/*
 * Cerebras Inference API
 *
 * The Cerebras Inference API offers developers a low-latency solution for AI model inference  powered by Cerebras Wafer-Scale Engines and CS-3 systems. The API provides access to  high-performance language models with unprecedented speed for AI inference workloads.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cerebras.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeInfo {
    /// Time spent in queue (seconds)
    #[serde(rename = "queue_time", skip_serializing_if = "Option::is_none")]
    pub queue_time: Option<f64>,
    /// Time spent processing prompt (seconds)
    #[serde(rename = "prompt_time", skip_serializing_if = "Option::is_none")]
    pub prompt_time: Option<f64>,
    /// Time spent generating completion (seconds)
    #[serde(rename = "completion_time", skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    /// Total time for the request (seconds)
    #[serde(rename = "total_time", skip_serializing_if = "Option::is_none")]
    pub total_time: Option<f64>,
    /// Unix timestamp when the response was created
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i32>,
}

impl TimeInfo {
    pub fn new() -> TimeInfo {
        TimeInfo {
            queue_time: None,
            prompt_time: None,
            completion_time: None,
            total_time: None,
            created: None,
        }
    }
}
