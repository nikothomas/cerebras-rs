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
pub struct Model {
    /// The model identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    /// Unix timestamp of when the model was created
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i32>,
    /// Organization that owns the model
    #[serde(rename = "owned_by", skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            id: None,
            object: None,
            created: None,
            owned_by: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "model")]
    Model,
}

impl Default for Object {
    fn default() -> Object {
        Self::Model
    }
}
