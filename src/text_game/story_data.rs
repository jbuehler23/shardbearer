//! Serializable story data structures for JSON loading

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryData {
    pub nodes: HashMap<String, StoryNodeData>,
    pub start: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNodeData {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascii_art: Option<String>,
    pub body: String,
    pub choices: Vec<ChoiceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending: Option<EndingData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceData {
    pub text: String,
    pub next: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndingData {
    pub code: String,
    pub name: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryMetadata {
    pub id: String,
    pub title: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub start_node: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryFlowData {
    pub start: StoryNodeData,
    pub paths: HashMap<String, HashMap<String, StoryNodeData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_nodes: Option<HashMap<String, StoryNodeData>>,
    pub endings: HashMap<String, StoryNodeData>,
}