use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Story {
    pub nodes: HashMap<String, StoryNode>,
    pub start: String,
}

#[derive(Clone)]
pub struct StoryNode {
    pub id: String,
    pub title: String,
    pub ascii: Option<String>,
    pub body: String,
    pub choices: Vec<Choice>,
    pub ending: Option<Ending>,
}

#[derive(Clone)]
pub struct Choice {
    pub text: String,
    pub next: String,
}

#[derive(Clone)]
pub struct Ending {
    pub code: String,
    pub name: String,
    pub summary: String,
}

pub const BANNER: &str = r#"
    ███████╗██╗  ██╗ █████╗ ██████╗ ██████╗ ██████╗ ███████╗ █████╗ ██████╗ ███████╗██████╗
    ██╔════╝██║  ██║██╔══██╗██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔════╝██╔══██╗
    ███████╗███████║███████║██████╔╝██║  ██║██████╔╝█████╗  ███████║██████╔╝█████╗  ██████╔╝
    ╚════██║██╔══██║██╔══██║██╔══██╗██║  ██║██╔══██╗██╔══╝  ██╔══██║██╔══██╗██╔══╝  ██╔══██╗
    ███████║██║  ██║██║  ██║██║  ██║██████╔╝██████╔╝███████╗██║  ██║██║  ██║███████╗██║  ██║
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝

             ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
             ▓                  A   N E W   C A R T H A G E   S T O R Y              ▓
             ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
"#;

pub const HELP_TEXT: &str = r#"
Commands: number to choose, or:
  help      - show this help
  back      - go to previous scene (if possible)
  history   - show visited nodes
  endings   - show unlocked endings
  quit      - exit the game
"#;

pub fn plugin(app: &mut App) {
    app.insert_resource(build_story());
}

pub fn build_story() -> Story {
    build_story_from_json()
}

/// Load story from JSON files
pub fn build_story_from_json() -> Story {
    use super::story_data::StoryFlowData;

    let mut nodes = HashMap::new();

    // Load ASCII art
    let ascii_art = load_ascii_art();

    // Load the organized story flow
    let story_flow_content = include_str!("../../assets/stories/shardbearer/story_flow.json");
    match serde_json::from_str::<StoryFlowData>(story_flow_content) {
        Ok(story_flow) => {
            // Helper function to convert StoryNodeData to StoryNode
            let convert_node = |node_data: super::story_data::StoryNodeData| -> StoryNode {
                let ascii = node_data.ascii_art.as_ref()
                    .and_then(|key| ascii_art.get(key))
                    .map(|s| s.clone());

                StoryNode {
                    id: node_data.id,
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
            };

            // Add start node
            nodes.insert("start".to_string(), convert_node(story_flow.start));

            // Add all path nodes
            for (_path_name, path_nodes) in story_flow.paths {
                for (node_id, node_data) in path_nodes {
                    nodes.insert(node_id, convert_node(node_data));
                }
            }

            // Add shared nodes if they exist
            if let Some(shared_nodes) = story_flow.shared_nodes {
                for (node_id, node_data) in shared_nodes {
                    nodes.insert(node_id, convert_node(node_data));
                }
            }

            // Add ending nodes
            for (node_id, node_data) in story_flow.endings {
                nodes.insert(node_id, convert_node(node_data));
            }
        }
        Err(e) => {
            eprintln!("Failed to parse story flow: {}", e);
        }
    }

    Story {
        nodes,
        start: "start".to_string(),
    }
}

fn load_ascii_art() -> HashMap<String, String> {
    let ascii_content = include_str!("../../assets/stories/shardbearer/ascii_art.json");
    serde_json::from_str(ascii_content).unwrap_or_default()
}