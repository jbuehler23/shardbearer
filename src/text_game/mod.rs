pub mod game;
pub mod story;
pub mod terminal;
// pub mod crt_effect; // Commented out - keeping file for future use

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        game::plugin,
        story::plugin,
        terminal::plugin,
    ));
}