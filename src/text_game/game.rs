use bevy::prelude::*;
use std::collections::HashSet;

use crate::text_game::story::{HELP_TEXT, Story};

#[derive(Resource, Default)]
pub struct Game {
    pub current: &'static str,
    pub should_redraw: bool,
    pub ended: bool,
    pub quit: bool,
    pub history: Vec<&'static str>,
    pub message: Option<String>,
    pub endings_unlocked: HashSet<&'static str>,
}

#[derive(Event, Clone, Message)]
pub struct InputEvent(pub String);

#[derive(Resource, Default)]
pub struct GameUI {
    pub display_text: String,
    pub input_buffer: String,
    pub awaiting_input: bool,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Game {
        current: "start",
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
        let input = line.trim().to_lowercase();

        // Global commands
        match input.as_str() {
            "help" => {
                game.message = Some(HELP_TEXT.trim().to_string());
                game.should_redraw = true;
                continue;
            }
            "back" => {
                if let Some(prev) = game.history.pop() {
                    game.current = prev;
                    game.ended = false;
                    game.should_redraw = true;
                } else {
                    game.message = Some("no previous scene".to_string());
                    game.should_redraw = true;
                }
                continue;
            }
            "history" => {
                let hist = if game.history.is_empty() {
                    "<empty>".to_string()
                } else {
                    game.history.join(" -> ")
                };
                game.message = Some(format!("visited: {}", hist));
                game.should_redraw = true;
                continue;
            }
            "endings" => {
                let msg = if game.endings_unlocked.is_empty() {
                    "none yet".to_string()
                } else {
                    let mut list: Vec<_> = game.endings_unlocked.iter().cloned().collect();
                    list.sort();
                    format!("{}", list.join(", "))
                };
                game.message = Some(format!("unlocked endings: {}", msg));
                game.should_redraw = true;
                continue;
            }
            "quit" | "exit" => {
                game.quit = true;
                continue;
            }
            _ => {}
        }

        // Scene-specific handling
        if let Some(node) = story.nodes.get(game.current) {
            if node.ending.is_some() {
                // End-screen menu
                match input.as_str() {
                    "1" | "reboot" => {
                        game.history.clear();
                        game.current = story.start;
                        game.ended = false;
                        game.message = Some("Back to the alley...".to_string());
                        game.should_redraw = true;
                    }
                    "2" | "endings" => {
                        let mut list: Vec<_> = game.endings_unlocked.iter().cloned().collect();
                        list.sort();
                        let msg = if list.is_empty() {
                            "none yet".to_string()
                        } else {
                            format!("{}", list.join(", "))
                        };
                        game.message = Some(format!("unlocked endings: {}", msg));
                        game.should_redraw = true;
                    }
                    "3" | "quit" | "exit" => {
                        game.quit = true;
                    }
                    _ => {
                        game.message = Some("choose 1-3".to_string());
                        game.should_redraw = true;
                    }
                }
            } else {
                // Normal node: parse numeric choice
                match input.parse::<usize>() {
                    Ok(n) if n >= 1 && n <= node.choices.len() => {
                        let choice = &node.choices[n - 1];
                        let prev_current = game.current;
                        game.history.push(prev_current);
                        game.current = choice.next;
                        game.should_redraw = true;

                        if let Some(next) = story.nodes.get(choice.next) {
                            if let Some(end) = &next.ending {
                                game.ended = true;
                                game.endings_unlocked.insert(end.code);
                            } else {
                                game.ended = false;
                            }
                        }
                    }
                    _ => {
                        game.message = Some("enter a valid number or 'help'".to_string());
                        game.should_redraw = true;
                    }
                }
            }
        } else {
            game.message = Some(format!("unknown scene: {}", game.current));
            game.should_redraw = true;
        }
    }
}
