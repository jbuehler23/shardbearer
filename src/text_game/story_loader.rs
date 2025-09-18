//! Story loader that reads JSON files and converts to game structures

use bevy::prelude::*;
use std::collections::HashMap;

use super::story::{Story, StoryNode, Choice, Ending};
use super::story_data::{StoryData, StoryNodeData, ChoiceData, EndingData};

pub struct StoryLoader;

impl StoryLoader {
    /// Load a story from JSON files
    pub fn load_story(_asset_server: &AssetServer, _story_id: &str) -> Result<Story, String> {
        // For now, we'll use the hardcoded story as a fallback
        // In a full implementation, this would load from the asset system
        Ok(super::story::build_story())
    }

    /// Convert loaded JSON data to game structures
    pub fn convert_story_data(data: StoryData, ascii_art: HashMap<String, String>) -> Story {
        let mut nodes = HashMap::new();

        for (id, node_data) in data.nodes {
            let ascii = node_data.ascii_art.as_ref()
                .and_then(|key| ascii_art.get(key))
                .map(|s| s.clone());

            nodes.insert(
                id.clone(),
                StoryNode {
                    id: id.clone(),
                    title: node_data.title,
                    ascii,
                    body: node_data.body,
                    choices: node_data.choices.into_iter()
                        .map(|c| Choice {
                            text: c.text,
                            next: c.next,
                        })
                        .collect(),
                    ending: node_data.ending.map(|e| Ending {
                        code: e.code,
                        name: e.name,
                        summary: e.summary,
                    }),
                }
            );
        }

        Story {
            nodes,
            start: data.start,
        }
    }

    /// Create a full story JSON structure from the current hardcoded story
    pub fn export_story_to_json(story: &Story) -> StoryData {
        let mut nodes = HashMap::new();

        for (id, node) in &story.nodes {
            nodes.insert(
                id.clone(),
                StoryNodeData {
                    id: id.clone(),
                    title: node.title.clone(),
                    ascii_art: node.ascii.as_ref().map(|s| {
                        // Map ASCII art to keys
                        if s.contains("BACK-ALLEY TERMINAL") {
                            "terminal".to_string()
                        } else if s.contains("Helix Spire") {
                            "spire".to_string()
                        } else if s.contains("Rain slicks") {
                            "alley".to_string()
                        } else {
                            "unknown".to_string()
                        }
                    }),
                    body: node.body.clone(),
                    choices: node.choices.iter()
                        .map(|c| ChoiceData {
                            text: c.text.clone(),
                            next: c.next.clone(),
                        })
                        .collect(),
                    ending: node.ending.as_ref().map(|e| EndingData {
                        code: e.code.clone(),
                        name: e.name.clone(),
                        summary: e.summary.clone(),
                    }),
                }
            );
        }

        StoryData {
            nodes,
            start: story.start.clone(),
        }
    }
}