use bevy::prelude::*;
use std::collections::HashSet;

use crate::text_game::story::{HELP_TEXT, Story};

#[derive(Resource, Default)]
pub struct Game {
    pub current: String,
    pub should_redraw: bool,
    pub ended: bool,
    pub quit: bool,
    pub history: Vec<String>,
    pub message: Option<String>,
    pub endings_unlocked: HashSet<String>,
}

#[derive(Event, Clone, Message)]
pub struct InputEvent(pub String);

#[derive(Resource, Default)]
pub struct GameUI {
    pub display_text: String,
    pub input_buffer: String,
    pub awaiting_input: bool,
    pub selected_choice: Option<usize>,
    pub num_choices: usize,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Game {
        current: "start".to_string(),
        should_redraw: true,
        ..Default::default()
    });
    app.insert_resource(GameUI::default());
    app.add_message::<InputEvent>();
    app.add_systems(Startup, startup_game);
    app.add_systems(Update, apply_input_system);
}

fn startup_game(mut game: ResMut<Game>) {
    game.message = Some("Neural link established. Type 'help' for command protocols.".to_string());
}

fn apply_input_system(
    mut ev_input: MessageReader<InputEvent>,
    story: Res<Story>,
    mut game: ResMut<Game>,
) {
    for InputEvent(line) in ev_input.read() {
        let cmd = line.trim().to_lowercase();

        // Special commands
        if cmd == "help" {
            game.message = Some(HELP_TEXT.to_string());
            game.should_redraw = true;
            continue;
        }

        if cmd == "quit" || cmd == "exit" {
            game.quit = true;
            continue;
        }

        if cmd == "history" {
            if game.history.is_empty() {
                game.message = Some("No history yet.".to_string());
            } else {
                let history_text = game.history.join(" → ");
                game.message = Some(format!("Path taken: {}", history_text));
            }
            game.should_redraw = true;
            continue;
        }

        if cmd == "back" {
            if game.history.len() > 1 {
                game.history.pop(); // Remove current
                if let Some(prev) = game.history.last() {
                    game.current = prev.clone();
                    game.message = Some("Stepped back.".to_string());
                    game.should_redraw = true;
                }
            } else {
                game.message = Some("Cannot go back further.".to_string());
                game.should_redraw = true;
            }
            continue;
        }

        if cmd == "endings" {
            if game.endings_unlocked.is_empty() {
                game.message = Some("No endings unlocked yet.".to_string());
            } else {
                let endings = game.endings_unlocked.iter()
                    .map(|e| e.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                game.message = Some(format!("Unlocked endings: {}", endings));
            }
            game.should_redraw = true;
            continue;
        }

        // Handle choices
        if let Some(current_node) = story.nodes.get(&game.current) {
            // Check for ending options
            if current_node.ending.is_some() {
                match cmd.as_str() {
                    "1" => {
                        // Restart from beginning
                        game.current = story.start.clone();
                        game.history.clear();
                        let current = game.current.clone();
                        game.history.push(current);
                        game.message = Some("SYSTEM REBOOT... Neural link re-established.".to_string());
                        game.should_redraw = true;
                    }
                    "2" => {
                        // Show endings
                        if game.endings_unlocked.is_empty() {
                            game.message = Some("No endings unlocked yet. Keep exploring!".to_string());
                        } else {
                            let endings = game.endings_unlocked.iter()
                                .map(|e| format!("  • {}", e))
                                .collect::<Vec<_>>()
                                .join("\n");
                            game.message = Some(format!("═══ ENDINGS UNLOCKED ═══\n{}", endings));
                        }
                        game.should_redraw = true;
                    }
                    "3" => {
                        game.quit = true;
                    }
                    _ => {
                        game.message = Some("Invalid choice. Enter 1, 2, or 3.".to_string());
                        game.should_redraw = true;
                    }
                }
            } else {
                // Regular choice handling
                if let Ok(choice_num) = cmd.parse::<usize>() {
                    if choice_num > 0 && choice_num <= current_node.choices.len() {
                        let choice = &current_node.choices[choice_num - 1];
                        let next_id = choice.next.clone();

                        if story.nodes.contains_key(&next_id) {
                            game.current = next_id.clone();

                            // Add to history only if not already at this node
                            if game.history.last() != Some(&game.current) {
                                let current = game.current.clone();
                        game.history.push(current);
                            }

                            // Check if new node has an ending
                            if let Some(next_node) = story.nodes.get(&game.current) {
                                if let Some(ending) = &next_node.ending {
                                    game.endings_unlocked.insert(ending.name.clone());
                                    game.ended = true;
                                }
                            }

                            game.should_redraw = true;
                        } else {
                            game.message = Some(format!("ERROR: Story node '{}' not found!", next_id));
                            game.should_redraw = true;
                        }
                    } else {
                        game.message = Some(format!(
                            "Invalid choice. Enter a number between 1 and {}.",
                            current_node.choices.len()
                        ));
                        game.should_redraw = true;
                    }
                } else {
                    game.message = Some("Invalid input. Enter a number or 'help' for commands.".to_string());
                    game.should_redraw = true;
                }
            }
        }
    }
}