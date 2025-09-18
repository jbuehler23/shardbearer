pub mod game;
pub mod story;
pub mod terminal;
// pub mod crt_effect; // Commented out - keeping file for future use
pub mod story_data;
pub mod story_loader;
// Backup modules (can be removed after confirming everything works)
// pub mod game_old;
// pub mod story_old;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        game::plugin,
        story::plugin,
        terminal::plugin,
    ));
}